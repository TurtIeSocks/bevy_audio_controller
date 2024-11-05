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

/// Stores all of the settings for a channel
///
/// It is recommended to mutate this resource via the [crate::events::SettingsEvent] event
///
/// Compared to calling the resource directly, but either is supported
#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub struct ChannelSettings<Channel: ACBounds> {
    channel_volume: Volume,
    track_settings: HashMap<AudioFiles, PlaybackSettings>,
    track_delay_modes: HashMap<AudioFiles, DelayMode>,
    default_settings: PlaybackSettings,
    default_delay_mode: DelayMode,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<Channel>,
}

impl<T: ACBounds> ChannelSettings<T> {
    /// Returns the volume of the channel on a scale of 0.0 - 1.0
    pub fn get_channel_volume(&self) -> f32 {
        self.channel_volume.get()
    }

    /// Sets the volume of the channel, scale is 0.0 - 1.0
    pub fn set_channel_volume(&mut self, volume: f32) {
        self.channel_volume = Volume::new(volume);
    }

    /// Returns the [PlaybackSettings] for a specific track
    ///
    /// or the default settings if the track does not have any settings for this channel
    pub fn get_track_setting(&self, id: &AudioFiles) -> PlaybackSettings {
        self.track_settings
            .get(id)
            .map_or(self.default_settings, |settings| settings.clone())
    }

    /// Sets the [PlaybackSettings] for a specific track
    pub fn set_track_settings(&mut self, id: AudioFiles, settings: PlaybackSettings) {
        self.track_settings.insert(id, settings);
    }

    /// Sets the [PlaybackSettings] for all tracks in this channel that exist in your asset folder
    pub fn set_all_track_settings(&mut self, settings: PlaybackSettings) {
        for track in ALL_FILES {
            self.track_settings.insert(track, settings.clone());
        }
    }

    /// Returns the [DelayMode] for a specific track
    pub fn get_track_delay_mode(&self, id: &AudioFiles) -> DelayMode {
        self.track_delay_modes
            .get(id)
            .map_or(self.default_delay_mode, |mode| mode.clone())
    }

    /// Sets the [DelayMode] for a specific track
    pub fn set_track_delay_mode(&mut self, id: AudioFiles, delay_mode: DelayMode) {
        self.track_delay_modes.insert(id, delay_mode);
    }

    /// Sets the [DelayMode] for all tracks in this channel that exist in your asset folder
    pub fn set_all_track_delay_modes(&mut self, delay_mode: DelayMode) {
        for track in ALL_FILES {
            self.track_delay_modes.insert(track, delay_mode.clone());
        }
    }

    /// Returns the default [PlaybackSettings] for this channel
    pub fn get_default_settings(&self) -> PlaybackSettings {
        self.default_settings.clone()
    }

    /// Sets the default [PlaybackSettings] for this channel
    pub fn set_default_settings(&mut self, settings: PlaybackSettings) {
        self.default_settings = settings;
    }

    /// Returns the default [DelayMode] for this channel
    pub fn get_default_delay_mode(&self) -> DelayMode {
        self.default_delay_mode
    }

    /// Sets the default [DelayMode] for this channel
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
