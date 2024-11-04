use std::{marker::PhantomData, sync::Mutex};

use bevy::{
    audio::{PlaybackSettings, Volume},
    ecs::{component::Component, system::Resource},
    utils::hashbrown::HashMap,
};
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

use super::audio_files::AudioFiles;

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub struct ChannelSettings<T: Component + Default> {
    volume: Mutex<Volume>,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<T>,
}

impl<T: Component + Default> ChannelSettings<T> {
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
pub struct TrackSettings<T: Component + Default> {
    track_map: HashMap<AudioFiles, PlaybackSettings>,
    default: PlaybackSettings,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<T>,
}

impl<T: Component + Default> TrackSettings<T> {
    pub fn get_track_setting(&self, id: &AudioFiles) -> PlaybackSettings {
        self.track_map
            .get(id)
            .map_or(self.default, |settings| settings.clone())
    }

    pub(super) fn set(&mut self, id: AudioFiles, settings: PlaybackSettings) {
        self.track_map.insert(id, settings);
    }

    pub(super) fn set_all(&mut self, settings: PlaybackSettings) {
        for (_, track) in self.track_map.iter_mut() {
            *track = settings.clone();
        }
    }

    pub(super) fn set_default(&mut self, settings: PlaybackSettings) {
        self.default = settings;
    }
}
