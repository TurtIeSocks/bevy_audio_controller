use std::ops::Deref;

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
    events::{PlayEvent, TrackEvent, VolumeEvent},
    plugin::{DelayMode, GlobalAudioChannel, HasChannel},
    resources::{ChannelSettings, TrackSettings},
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
            .add_event::<VolumeEvent<Channel>>()
            .add_event::<TrackEvent<Channel>>()
            .init_resource::<ChannelSettings<Channel>>()
            .init_resource::<TrackSettings<Channel>>()
            .add_systems(
                Update,
                (
                    ecs_system::<Channel>,
                    update_volume_on_insert::<Channel>,
                    volume_event_reader::<Channel>.run_if(on_event::<VolumeEvent<Channel>>()),
                    track_event_reader::<Channel>.run_if(on_event::<TrackEvent<Channel>>()),
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
    global: Res<ChannelSettings<GlobalAudioChannel>>,
    track_query: Query<&AudioSink, With<Channel>>,
) {
    let volume = get_normalized_volume(channel, global);
    for sink in track_query.iter() {
        sink.set_volume(volume);
    }
}

fn update_volume_on_insert<Channel: Component + Default>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalAudioChannel>>,
    sink_query: Query<&AudioSink, (Added<AudioSink>, With<Channel>)>,
) {
    let volume = get_normalized_volume(channel, global);
    for sink in sink_query.iter() {
        sink.set_volume(sink.volume() * volume);
    }
}

fn volume_event_reader<Channel: Component + Default>(
    channel_settings: Res<ChannelSettings<Channel>>,
    mut events: EventReader<VolumeEvent<Channel>>,
) {
    for event in events.read() {
        channel_settings.set_volume(event.volume);
    }
    let _ = channel_settings.deref();
}

fn track_event_reader<Channel: Component + Default>(
    mut track_settings: ResMut<TrackSettings<Channel>>,
    mut events: EventReader<TrackEvent<Channel>>,
) {
    for event in events.read() {
        if let Some(id) = event.id {
            track_settings.set(id, event.settings);
        } else {
            track_settings.set_all(event.settings);
        }
    }
}

fn ecs_system<Channel: Component + Default>(
    mut commands: Commands,
    track_settings: Res<TrackSettings<Channel>>,
    asset_loader: Res<AssetLoader>,
    query: Query<
        (Entity, &AudioFiles, Option<&PlaybackSettings>, &DelayMode),
        (Added<Channel>, Without<AudioSink>),
    >,
    mut ew: EventWriter<PlayEvent<Channel>>,
) {
    let mut events = Vec::new();
    for (entity, audio_file, settings, mode) in query.iter() {
        if mode == &DelayMode::Immediate {
            commands.entity(entity).insert_audio_track(audio_file);
            if let Some(handler) = asset_loader.get(audio_file) {
                commands.entity(entity).insert(handler);
            }
            if settings.is_none() {
                commands
                    .entity(entity)
                    .insert(track_settings.get_track_setting(audio_file));
            }
        } else {
            let event = PlayEvent::<Channel>::new(*audio_file).with_entity(entity);
            if let Some(settings) = settings {
                events.push(event.with_settings(settings.clone()));
            } else {
                events.push(event);
            }
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
    track_settings: Res<TrackSettings<Channel>>,
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
            track_settings.get_track_setting(&event.id)
        };
        let delay_mode = if let Some(mode) = event.delay_mode {
            mode
        } else {
            DelayMode::Wait
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

fn get_normalized_volume<Channel: Component + Default>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalAudioChannel>>,
) -> f32 {
    channel.get_volume() * global.get_volume()
}
