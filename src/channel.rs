use bevy::{
    app::{PostUpdate, Update},
    audio::{AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings},
    ecs::{
        component::Component,
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
    utils::hashbrown::HashSet,
};

use crate::{
    // bounds::Bounds,
    ac_assets::AssetLoader,
    ac_traits::CommandAudioTracks,
    audio_files::AudioFiles,
    delay_mode::DelayMode,
    events::{PlayEvent, SettingsEvent},
    global_channel::GlobalChannel,
    helpers,
    plugin::HasChannel,
    resources::ChannelSettings,
};

pub trait ChannelRegistration {
    fn register_audio_channel<Channel: Component + Default>(&mut self) -> &mut Self;
}

impl ChannelRegistration for bevy::app::App {
    fn register_audio_channel<Channel: Component + Default>(&mut self) -> &mut Self {
        self.world_mut()
            .register_component_hooks::<Channel>()
            .on_add(|mut world, entity, _| {
                world.commands().entity(entity).insert(HasChannel);
            });

        self.add_event::<PlayEvent<Channel>>()
            .add_event::<SettingsEvent<Channel>>()
            .init_resource::<ChannelSettings<Channel>>()
            .add_systems(
                Update,
                (
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
            )
    }
}

fn update_track_volumes<Channel: Component + Default>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
    track_query: Query<&AudioSink, With<Channel>>,
) {
    let volume = helpers::get_normalized_volume(channel, global);
    for sink in track_query.iter() {
        sink.set_volume(volume);
    }
}

fn update_volume_on_insert<Channel: Component + Default>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
    sink_query: Query<&AudioSink, (Added<AudioSink>, With<Channel>)>,
) {
    let volume = helpers::get_normalized_volume(channel, global);
    for sink in sink_query.iter() {
        sink.set_volume(sink.volume() * volume);
    }
}

fn ecs_system<Channel: Component + Default>(
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

fn remove_audio_components<Channel: Component + Default>(
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

fn play_event_reader<Channel: Component + Default>(
    mut commands: Commands,
    asset_loader: Res<AssetLoader>,
    mut events: EventReader<PlayEvent<Channel>>,
    channel_query: Query<(&AudioFiles, &AudioSink), With<Channel>>,
    channel_settings: Res<ChannelSettings<Channel>>,
) {
    let mut is_playing = channel_query
        .iter()
        .filter_map(
            |(file, sink)| {
                if sink.is_paused() {
                    None
                } else {
                    Some(file)
                }
            },
        )
        .collect::<HashSet<&AudioFiles>>();

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
        if delay_mode == DelayMode::Immediate || !is_playing.contains(&event.id) {
            is_playing.insert(&event.id);
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

fn settings_event_reader<Channel: Component + Default>(
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
