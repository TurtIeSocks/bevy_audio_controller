use std::{marker::PhantomData, sync::Mutex};

use bevy::{
    audio::{PlaybackSettings, Volume},
    ecs::{component::Component, system::Resource},
    utils::hashbrown::HashMap,
};
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

use crate::prelude::DelayMode;

use super::audio_files::AudioFiles;

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub struct ChannelSettings<Channel: Component + Default> {
    channel_volume: Mutex<Volume>,
    track_settings: HashMap<AudioFiles, PlaybackSettings>,
    default_settings: PlaybackSettings,
    default_delay_mode: DelayMode,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<Channel>,
}

impl<T: Component + Default> ChannelSettings<T> {
    pub fn get_channel_volume(&self) -> f32 {
        self.channel_volume.lock().unwrap().get()
    }

    pub(super) fn set_channel_volume(&self, volume: f32) {
        *self.channel_volume.lock().unwrap() = Volume::new(volume);
    }

    pub fn get_track_setting(&self, id: &AudioFiles) -> PlaybackSettings {
        self.track_settings
            .get(id)
            .map_or(self.default_settings, |settings| settings.clone())
    }

    pub(super) fn set_track_settings(&mut self, id: AudioFiles, settings: PlaybackSettings) {
        self.track_settings.insert(id, settings);
    }

    pub(super) fn set_all_track_settings(&mut self, settings: PlaybackSettings) {
        for track in self.track_settings.values_mut() {
            *track = settings.clone();
        }
    }

    pub fn get_default_settings(&self) -> PlaybackSettings {
        self.default_settings.clone()
    }

    pub(super) fn set_default_settings(&mut self, settings: PlaybackSettings) {
        self.default_settings = settings;
    }

    pub fn get_default_delay_mode(&self) -> DelayMode {
        self.default_delay_mode
    }

    pub(super) fn set_default_delay_mode(&mut self, delay_mode: DelayMode) {
        self.default_delay_mode = delay_mode;
    }
}
