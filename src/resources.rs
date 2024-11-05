use std::marker::PhantomData;

use bevy::{
    audio::{PlaybackSettings, Volume},
    ecs::system::Resource,
    time::{Timer, TimerMode},
    utils::hashbrown::HashMap,
};
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};

use crate::{bounds::ACBounds, prelude::DelayMode};

use super::audio_files::{AudioFiles, ALL_FILES};

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub struct ChannelSettings<Channel: ACBounds> {
    channel_volume: Volume,
    track_settings: HashMap<AudioFiles, PlaybackSettings>,
    default_settings: PlaybackSettings,
    default_delay_mode: DelayMode,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<Channel>,
}

impl<T: ACBounds> ChannelSettings<T> {
    pub fn get_channel_volume(&self) -> f32 {
        self.channel_volume.get()
    }

    pub fn set_channel_volume(&mut self, volume: f32) {
        self.channel_volume = Volume::new(volume);
    }

    pub fn get_track_setting(&self, id: &AudioFiles) -> PlaybackSettings {
        self.track_settings
            .get(id)
            .map_or(self.default_settings, |settings| settings.clone())
    }

    pub fn set_track_settings(&mut self, id: AudioFiles, settings: PlaybackSettings) {
        self.track_settings.insert(id, settings);
    }

    pub fn set_all_track_settings(&mut self, settings: PlaybackSettings) {
        for track in ALL_FILES {
            self.track_settings.insert(track, settings.clone());
        }
    }

    pub fn get_default_settings(&self) -> PlaybackSettings {
        self.default_settings.clone()
    }

    pub fn set_default_settings(&mut self, settings: PlaybackSettings) {
        self.default_settings = settings;
    }

    pub fn get_default_delay_mode(&self) -> DelayMode {
        self.default_delay_mode
    }

    pub fn set_default_delay_mode(&mut self, delay_mode: DelayMode) {
        self.default_delay_mode = delay_mode;
    }
}

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub(super) struct AudioCache<T: ACBounds> {
    pub(super) map: HashMap<AudioFiles, Timer>,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<T>,
}

impl<T: ACBounds> AudioCache<T> {
    pub(super) fn tick(&mut self, time: bevy::utils::Duration) {
        for timer in self.map.values_mut() {
            timer.tick(time);
        }
    }

    pub(super) fn can_play(&self, id: &AudioFiles) -> bool {
        self.map.get(id).map_or(true, |timer| timer.finished())
    }

    pub(super) fn set_entry(&mut self, id: AudioFiles, duration: f32) {
        self.map
            .insert(id, Timer::from_seconds(duration, TimerMode::Once));
    }
}
