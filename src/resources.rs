use std::{marker::PhantomData, sync::Mutex};

use bevy::{
    audio::{PlaybackSettings, Volume},
    ecs::system::Resource,
    utils::hashbrown::HashMap,
};
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

use super::{audio_files::AudioFiles, bounds::Bounds};

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub struct ChannelSettings<T: Bounds> {
    volume: Mutex<Volume>,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<T>,
}

impl<T: Bounds> ChannelSettings<T> {
    pub fn get_volume(&self) -> f32 {
        self.volume.lock().unwrap().get()
    }

    pub(super) fn set_volume(&self, volume: f32) {
        *self.volume.lock().unwrap() = Volume::new(volume);
    }
}

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub struct TrackSettings<T: Bounds> {
    track_map: HashMap<AudioFiles, PlaybackSettings>,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<T>,
}

impl<T: Bounds> TrackSettings<T> {
    pub fn get_track_setting(&self, id: &AudioFiles) -> PlaybackSettings {
        self.track_map
            .get(id)
            .map_or(PlaybackSettings::default(), |settings| settings.clone())
    }

    pub(super) fn set(&mut self, id: AudioFiles, settings: PlaybackSettings) {
        self.track_map.insert(id, settings);
    }

    pub(super) fn set_all(&mut self, settings: PlaybackSettings) {
        for (_, track) in self.track_map.iter_mut() {
            *track = settings.clone();
        }
    }
}
