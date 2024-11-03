use std::ops::Deref;

use bevy::{
    app::{PostUpdate, Update},
    audio::{AudioSink, AudioSinkPlayback, PlaybackSettings},
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Added, With},
        schedule::{
            common_conditions::{on_event, resource_changed},
            IntoSystemConfigs,
        },
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    // log::info,
    utils::hashbrown::HashSet,
};

use crate::{
    // bounds::Bounds,
    ac_assets::AssetLoader,
    ac_traits::InsertAudioTrack,
    audio_files::AudioFiles,
    events::{PlayEvent, TrackEvent, VolumeEvent},
    plugin::{GlobalAudioChannel, HasChannel},
    resources::{ChannelSettings, TrackSettings},
};

pub trait ChannelRegistration {
    fn register_audio_channel<Channel: Component + Default>(&mut self) -> &mut Self;
}

impl ChannelRegistration for bevy::app::App {
    fn register_audio_channel<Channel: Component + Default>(&mut self) -> &mut Self {
        self.add_event::<PlayEvent<Channel>>()
            .add_event::<VolumeEvent<Channel>>()
            .add_event::<TrackEvent<Channel>>()
            .init_resource::<ChannelSettings<Channel>>()
            .init_resource::<TrackSettings<Channel>>()
            .add_systems(
                Update,
                (
                    replace_file_enum::<Channel>,
                    update_volume_on_insert::<Channel>,
                    volume_event_reader::<Channel>.run_if(on_event::<VolumeEvent<Channel>>()),
                    track_event_reader::<Channel>.run_if(on_event::<TrackEvent<Channel>>()),
                    update_track_volumes::<Channel>
                        .run_if(resource_changed::<ChannelSettings<Channel>>),
                ),
            )
            .add_systems(
                PostUpdate,
                play_event_reader::<Channel>.run_if(on_event::<PlayEvent<Channel>>()),
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

fn replace_file_enum<Channel: Component + Default>(
    mut commands: Commands,
    track_settings: Res<TrackSettings<Channel>>,
    asset_loader: Res<AssetLoader>,
    query: Query<(Entity, &AudioFiles, Option<&PlaybackSettings>), Added<Channel>>,
) {
    for (entity, audio_file, settings) in query.iter() {
        commands
            .entity(entity)
            .remove::<AudioFiles>()
            .insert_audio_track(audio_file)
            .insert(HasChannel);
        if let Some(handler) = asset_loader.get(audio_file) {
            commands.entity(entity).insert(handler);
        }
        if settings.is_none() {
            commands
                .entity(entity)
                .insert(track_settings.get_track_setting(audio_file));
        }
    }
}

fn play_event_reader<Channel: Component + Default>(
    mut commands: Commands,
    asset_loader: Res<AssetLoader>,
    mut events: EventReader<PlayEvent<Channel>>,
    channel_query: Query<(&Name, &AudioSink), With<Channel>>,
    track_settings: Res<TrackSettings<Channel>>,
) {
    let mut is_playing = channel_query
        .iter()
        .filter_map(|(name, sink)| {
            if sink.is_paused() {
                None
            } else {
                Some(name.into())
            }
        })
        .collect::<HashSet<AudioFiles>>();

    for event in events.read() {
        if event.force || !is_playing.contains(&event.id) {
            let settings = if let Some(event_settings) = event.settings {
                event_settings
            } else {
                track_settings.get_track_setting(&event.id)
            };
            is_playing.insert(event.id);
            let id = event.id.to_string();
            if let Some(handler) = asset_loader.get(&event.id) {
                let bundle = (handler, settings, Channel::default());
                let audio_entity = if let Some(dest_entity) = event.entity {
                    commands.entity(dest_entity).insert(bundle).id()
                } else {
                    let child = commands
                        .spawn(bundle)
                        .insert((Name::new(id), HasChannel))
                        .id();
                    if let Some(parent_entity) = event.parent {
                        commands.entity(parent_entity).add_child(child);
                    }
                    child
                };
                commands.entity(audio_entity).insert_audio_track(&event.id);
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
