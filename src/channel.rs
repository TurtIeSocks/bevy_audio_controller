use std::ops::Deref;

use bevy::{
    app::Update,
    asset::AssetServer,
    audio::{AudioBundle, AudioSink, AudioSinkPlayback},
    core::Name,
    ecs::{
        event::EventReader,
        query::{Added, With},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    prelude::{on_event, resource_changed, IntoSystemConfigs},
    utils::hashbrown::HashSet,
};

use crate::{
    ac_traits::InsertAudioTrack,
    audio_files::AudioFiles,
    bounds::Bounds,
    events::{PlayEvent, TrackEvent, VolumeEvent},
    plugin::GlobalAudioChannel,
    resources::{ChannelSettings, TrackSettings},
};

pub trait ChannelRegistration {
    fn register_audio_channel<Channel: Bounds>(&mut self);
}

impl ChannelRegistration for bevy::app::App {
    fn register_audio_channel<Channel: Bounds>(&mut self) {
        self.add_event::<VolumeEvent<Channel>>()
            .add_event::<PlayEvent<Channel>>()
            .init_resource::<ChannelSettings<Channel>>()
            .init_resource::<TrackSettings<Channel>>()
            .add_systems(
                Update,
                (
                    update_volume_on_insert::<Channel>,
                    play_event_reader::<Channel>.run_if(on_event::<PlayEvent<Channel>>()),
                    volume_event_reader::<Channel>.run_if(on_event::<VolumeEvent<Channel>>()),
                    track_event_reader::<Channel>.run_if(on_event::<TrackEvent<Channel>>()),
                    update_track_volumes::<Channel>
                        .run_if(resource_changed::<ChannelSettings<Channel>>),
                ),
            );
    }
}

fn update_track_volumes<Channel: Bounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalAudioChannel>>,
    track_query: Query<&AudioSink, With<Channel>>,
) {
    let volume = get_normalized_volume(channel, global);
    for sink in track_query.iter() {
        sink.set_volume(volume);
    }
}

fn update_volume_on_insert<Channel: Bounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalAudioChannel>>,
    sink_query: Query<&AudioSink, (Added<AudioSink>, With<Channel>)>,
) {
    let volume = get_normalized_volume(channel, global);
    for sink in sink_query.iter() {
        sink.set_volume(sink.volume() * volume);
    }
}

fn volume_event_reader<Channel: Bounds>(
    channel_settings: Res<ChannelSettings<Channel>>,
    mut events: EventReader<VolumeEvent<Channel>>,
) {
    for event in events.read() {
        channel_settings.set_volume(event.volume);
    }
    channel_settings.deref();
}

fn track_event_reader<Channel: Bounds>(
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

fn play_event_reader<Channel: Bounds>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<PlayEvent<Channel>>,
    query: Query<(&Name, &AudioSink), With<Channel>>,
    track_settings: Res<TrackSettings<Channel>>,
) {
    let is_playing = query
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
        let id = event.id.to_string();
        if event.force || !is_playing.contains(&event.id) {
            let settings = if let Some(event_settings) = event.settings {
                event_settings
            } else {
                track_settings.get_track_setting(&event.id)
            };
            let child = commands
                .spawn((
                    AudioBundle {
                        settings,
                        source: asset_server.load(&id),
                    },
                    Channel::default(),
                ))
                .insert_audio_track(&id)
                .insert(Name::new(id))
                .id();
            if let Some(entity) = event.parent {
                commands.entity(entity).add_child(child);
            }
        }
    }
}

fn get_normalized_volume<Channel: Bounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalAudioChannel>>,
) -> f32 {
    channel.get_volume() * global.get_volume()
}
