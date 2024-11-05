use bevy::{
    app::{PostUpdate, Update},
    audio::{AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings},
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{Added, With},
        schedule::{
            common_conditions::{on_event, resource_changed},
            IntoSystemConfigs,
        },
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    prelude::{DespawnRecursiveExt, RemovedComponents, Without},
};

use crate::{
    ac_assets::ACAssetLoader,
    ac_traits::CommandAudioTracks,
    audio_files::AudioFiles,
    bounds::ACBounds,
    delay_mode::DelayMode,
    events::{PlayEvent, SettingsEvent},
    global::GlobalChannel,
    helpers,
    plugin::HasChannel,
    resources::{AudioCache, ChannelSettings},
};

pub trait ChannelRegistration {
    fn register_audio_channel<Channel: ACBounds>(&mut self) -> &mut Self;
}

impl ChannelRegistration for bevy::app::App {
    fn register_audio_channel<Channel: ACBounds>(&mut self) -> &mut Self {
        self.world_mut()
            .register_component_hooks::<Channel>()
            .on_add(|mut world, entity, _| {
                world.commands().entity(entity).insert(HasChannel);
            });

        self.add_event::<PlayEvent<Channel>>()
            .add_event::<SettingsEvent<Channel>>()
            .init_resource::<ChannelSettings<Channel>>()
            .init_resource::<AudioCache<Channel>>()
            .add_systems(
                Update,
                (
                    tick_audio_cache::<Channel>,
                    ecs_system::<Channel>,
                    update_volume_on_insert::<Channel>,
                    settings_event_reader::<Channel>.run_if(on_event::<SettingsEvent<Channel>>()),
                    update_track_volumes::<Channel>
                        .run_if(resource_changed::<ChannelSettings<Channel>>),
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    remove_audio_components::<Channel>,
                    play_event_reader::<Channel>.run_if(on_event::<PlayEvent<Channel>>()),
                ),
            );

        #[cfg(feature = "inspect")]
        self.register_type::<Channel>()
            .register_type::<ChannelSettings<Channel>>();

        self
    }
}

fn tick_audio_cache<Channel: ACBounds>(
    mut cache: ResMut<AudioCache<Channel>>,
    time: Res<bevy::time::Time>,
) {
    cache.tick(time.delta());
}

fn update_track_volumes<Channel: ACBounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
    track_query: Query<&AudioSink, With<Channel>>,
) {
    let volume = helpers::get_normalized_volume(channel, global);
    for sink in track_query.iter() {
        sink.set_volume(volume);
    }
}

fn update_volume_on_insert<Channel: ACBounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
    sink_query: Query<&AudioSink, (Added<AudioSink>, With<Channel>)>,
) {
    let volume = helpers::get_normalized_volume(channel, global);
    for sink in sink_query.iter() {
        sink.set_volume(sink.volume() * volume);
    }
}

fn ecs_system<Channel: ACBounds>(
    query: Query<
        (Entity, &AudioFiles, Option<&PlaybackSettings>, &DelayMode),
        (Added<Channel>, Without<AudioSink>),
    >,
    mut ew: EventWriter<PlayEvent<Channel>>,
) {
    let mut events = Vec::new();
    for (entity, audio_file, settings, mode) in query.iter() {
        let event = PlayEvent::<Channel>::new(*audio_file)
            .with_entity(entity)
            .with_delay_mode(mode.clone());
        if let Some(settings) = settings {
            events.push(event.with_settings(settings.clone()));
        } else {
            events.push(event);
        }
    }
    ew.send_batch(events);
}

fn remove_audio_components<Channel: ACBounds>(
    mut commands: Commands,
    mut removed: RemovedComponents<AudioSink>,
    channel_query: Query<&AudioFiles, With<Channel>>,
) {
    for entity in removed.read() {
        if let Ok(track) = channel_query.get(entity) {
            commands
                .entity(entity)
                .remove_audio_track(track)
                .remove::<(Channel, AudioFiles)>();
        }
    }
}

fn play_event_reader<Channel: ACBounds>(
    mut commands: Commands,
    asset_loader: Res<ACAssetLoader>,
    mut events: EventReader<PlayEvent<Channel>>,
    channel_settings: Res<ChannelSettings<Channel>>,
    mut audio_cache: ResMut<AudioCache<Channel>>,
) {
    for event in events.read() {
        let settings = if let Some(event_settings) = event.settings {
            event_settings
        } else {
            channel_settings.get_track_setting(&event.id)
        };
        let delay_mode = if let Some(mode) = event.delay_mode {
            mode
        } else {
            channel_settings.get_default_delay_mode()
        };
        let can_play = audio_cache.can_play(&event.id);
        if delay_mode == DelayMode::Immediate || can_play {
            if can_play {
                let next_delay = delay_mode.get_delay(event.id.duration());
                audio_cache.set_entry(event.id, next_delay);
            }
            if let Some(handler) = asset_loader.get(&event.id) {
                let bundle = (handler, settings, event.id, Channel::default());
                if let Some(dest_entity) = event.entity {
                    if event.child {
                        let child = commands.spawn(bundle).id();
                        commands.entity(dest_entity).add_child(child);
                    } else {
                        commands.entity(dest_entity).insert(bundle);
                    }
                } else {
                    commands.spawn(bundle);
                }
            }
        } else if let Some(entity) = event.entity {
            match settings.mode {
                PlaybackMode::Despawn => {
                    if event.child {
                        return;
                    }
                    commands.entity(entity).despawn_recursive();
                }
                PlaybackMode::Remove => {
                    commands
                        .entity(entity)
                        .remove::<(Channel, PlaybackSettings, AudioFiles)>();
                }
                _ => {}
            }
        }
    }
}

fn settings_event_reader<Channel: ACBounds>(
    mut channel_settings: ResMut<ChannelSettings<Channel>>,
    mut events: EventReader<SettingsEvent<Channel>>,
) {
    for event in events.read() {
        if let Some(delay_mode) = event.delay_mode {
            channel_settings.set_default_delay_mode(delay_mode);
        }
        if let Some(volume) = event.volume {
            channel_settings.set_channel_volume(volume);
        }
        if let Some(settings) = event.settings {
            if let Some(id) = event.track {
                channel_settings.set_track_settings(id, settings);
            } else if event.all {
                channel_settings.set_all_track_settings(settings);
            } else {
                channel_settings.set_default_settings(settings);
            }
        }
    }
}
