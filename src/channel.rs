use bevy::{
    app::{App, PostUpdate, Update},
    audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings},
    ecs::{
        entity::Entity,
        lifecycle::RemovedComponents,
        message::{MessageReader, MessageWriter},
        query::{Added, With, Without},
        schedule::{
            IntoScheduleConfigs, SystemCondition,
            common_conditions::{on_message, resource_changed},
        },
        system::{Commands, Query, Res, ResMut},
    },
    time::Time,
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

impl ChannelRegistration for App {
    /// Registers an audio channel to the Bevy app
    fn register_audio_channel<Channel: ACBounds>(&mut self) -> &mut Self {
        self.world_mut()
            .register_component_hooks::<Channel>()
            .on_add(|mut world, ctx| {
                world.commands().entity(ctx.entity).insert(HasChannel);
            });

        self.add_message::<PlayEvent<Channel>>()
            .add_message::<SettingsEvent<Channel>>()
            .init_resource::<ChannelSettings<Channel>>()
            .init_resource::<AudioCache<Channel>>()
            .add_systems(
                Update,
                (
                    tick_audio_cache::<Channel>,
                    ecs_system::<Channel>,
                    // update_internal_timer_on_speed_change::<Channel>,
                    update_volume_on_insert::<Channel>,
                    settings_event_reader::<Channel>.run_if(on_message::<SettingsEvent<Channel>>),
                    // Also re-apply when the global channel changes: its volume is a
                    // master multiplier, so a global change must propagate to every
                    // channel's live tracks, not just tracks on the global channel.
                    update_track_volumes::<Channel>.run_if(
                        resource_changed::<ChannelSettings<Channel>>
                            .or_eager(resource_changed::<ChannelSettings<GlobalChannel>>),
                    ),
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    remove_audio_components::<Channel>,
                    play_event_reader::<Channel>.run_if(on_message::<PlayEvent<Channel>>),
                ),
            );

        #[cfg(feature = "inspect")]
        self.register_type::<Channel>()
            .register_type::<ChannelSettings<Channel>>()
            .register_type::<AudioCache<Channel>>();

        self
    }
}

fn tick_audio_cache<Channel: ACBounds>(mut cache: ResMut<AudioCache<Channel>>, time: Res<Time>) {
    cache.tick(time.delta());
}

fn update_track_volumes<Channel: ACBounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
    mut track_query: Query<(&mut AudioSink, &AudioFiles), With<Channel>>,
) {
    let volume = helpers::get_normalized_volume(&channel, &global);
    track_query.iter_mut().for_each(|(mut sink, id)| {
        let track_volume = channel.get_track_setting(id).volume;
        sink.set_volume(volume * track_volume);
    });
}

fn update_volume_on_insert<Channel: ACBounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
    mut sink_query: Query<&mut AudioSink, (Added<AudioSink>, With<Channel>)>,
) {
    let volume = helpers::get_normalized_volume(&channel, &global);
    sink_query.iter_mut().for_each(|mut sink| {
        let new_volume = sink.volume() * volume;
        #[cfg(feature = "log")]
        if volume.to_linear() != 1.0 {
            bevy::log::trace!(
                "Setting volume from {} to {}",
                volume.to_linear(),
                new_volume.to_linear()
            );
        }
        sink.set_volume(new_volume);
    });
}

fn ecs_system<Channel: ACBounds>(
    query: Query<
        (Entity, &AudioFiles, Option<&PlaybackSettings>, &DelayMode),
        (Added<Channel>, Without<AudioSink>),
    >,
    mut ew: MessageWriter<PlayEvent<Channel>>,
) {
    let mut events = Vec::new();
    query
        .iter()
        .for_each(|(entity, audio_file, settings, mode)| {
            let event = PlayEvent::<Channel>::new(*audio_file)
                .with_entity(entity)
                .with_delay_mode(mode.clone());
            if let Some(settings) = settings {
                events.push(event.with_settings(settings.clone()));
            } else {
                events.push(event);
            }
        });
    ew.write_batch(events);
}

fn remove_audio_components<Channel: ACBounds>(
    mut commands: Commands,
    mut removed: RemovedComponents<AudioSink>,
    channel_query: Query<&AudioFiles, With<Channel>>,
) {
    removed.read().for_each(|entity| {
        if let Ok(track) = channel_query.get(entity) {
            commands
                .entity(entity)
                .remove_audio_track(track)
                .remove::<(Channel, AudioFiles)>();
        }
    });
}

fn play_event_reader<Channel: ACBounds>(
    mut commands: Commands,
    asset_loader: Res<ACAssetLoader>,
    mut events: MessageReader<PlayEvent<Channel>>,
    channel_settings: Res<ChannelSettings<Channel>>,
    mut audio_cache: ResMut<AudioCache<Channel>>,
) {
    events.read().for_each(|event| {
        let settings = if let Some(event_settings) = event.settings {
            event_settings
        } else {
            channel_settings.get_track_setting(&event.id)
        };
        let delay_mode = if let Some(mode) = event.delay_mode {
            mode
        } else {
            channel_settings.get_track_delay_mode(&event.id)
        };
        let can_play = audio_cache.can_play(&event.id);
        if delay_mode == DelayMode::Immediate || can_play {
            if can_play {
                let next_delay = delay_mode.get_delay(event.id.duration() / settings.speed);
                audio_cache.set_entry(event.id, next_delay);
            }
            if let Some(handler) = asset_loader.get(&event.id) {
                let bundle = (
                    AudioPlayer::new(handler),
                    settings,
                    event.id,
                    Channel::default(),
                );
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
                    commands.entity(entity).despawn();
                }
                PlaybackMode::Remove => {
                    commands
                        .entity(entity)
                        .remove::<(Channel, PlaybackSettings, AudioFiles)>();
                }
                _ => {}
            }
        }
    });
}

fn settings_event_reader<Channel: ACBounds>(
    mut channel_settings: ResMut<ChannelSettings<Channel>>,
    mut events: MessageReader<SettingsEvent<Channel>>,
) {
    events.read().for_each(|event| {
        if let Some(volume) = event.volume {
            channel_settings.set_channel_volume(volume);
        }
        if let Some(id) = event.track {
            if let Some(delay_mode) = event.delay_mode {
                channel_settings.set_track_delay_mode(id, delay_mode);
            }
            if let Some(settings) = event.settings {
                channel_settings.set_track_settings(id, settings);
            }
        } else if event.all {
            if let Some(delay_mode) = event.delay_mode {
                channel_settings.set_all_track_delay_modes(delay_mode);
            }
            if let Some(settings) = event.settings {
                channel_settings.set_all_track_settings(settings);
            }
        } else {
            if let Some(delay_mode) = event.delay_mode {
                channel_settings.set_default_delay_mode(delay_mode);
            }
            if let Some(settings) = event.settings {
                channel_settings.set_default_settings(settings);
            }
        }
    });
}
