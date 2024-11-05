use std::marker::PhantomData;

use bevy::{
    audio::PlaybackSettings,
    ecs::{entity::Entity, event::Event},
};

use crate::{audio_files::AudioFiles, bounds::Bounds, delay_mode::DelayMode};

#[derive(Event)]
pub struct PlayEvent<T: Bounds> {
    pub(super) id: AudioFiles,
    pub(super) entity: Option<Entity>,
    pub(super) child: bool,
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) delay_mode: Option<DelayMode>,
    _marker: PhantomData<T>,
}

impl<T: Bounds> PlayEvent<T> {
    pub fn new(id: AudioFiles) -> Self {
        Self {
            id,
            entity: None,
            settings: None,
            delay_mode: None,
            child: false,
            _marker: PhantomData::<T>,
        }
    }

    pub fn with_entity(self, entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..self
        }
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn with_delay_mode(mut self, delay_mode: DelayMode) -> Self {
        self.delay_mode = Some(delay_mode);
        self
    }

    pub fn as_child(self) -> Self {
        Self {
            child: true,
            ..self
        }
    }
}

impl<Channel: Bounds> From<AudioFiles> for PlayEvent<Channel> {
    fn from(id: AudioFiles) -> Self {
        Self::new(id)
    }
}

#[derive(Event)]
pub struct SettingsEvent<Channel: Bounds> {
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) volume: Option<f32>,
    pub(super) track: Option<AudioFiles>,
    pub(super) delay_mode: Option<DelayMode>,
    pub(super) all: bool,
    _marker: PhantomData<Channel>,
}

impl<Channel: Bounds> SettingsEvent<Channel> {
    pub fn new() -> Self {
        Self {
            track: None,
            settings: None,
            volume: None,
            delay_mode: None,
            all: false,
            _marker: PhantomData::<Channel>,
        }
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn with_track(mut self, id: AudioFiles) -> Self {
        if self.all {
            panic!("Do set all and a specific track at the same time, either call `all()` or `with_track()`");
        }
        self.track = Some(id);
        self
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = Some(volume);
        self
    }

    pub fn all(mut self) -> Self {
        if self.track.is_some() {
            panic!("Do set all and a specific track at the same time, either call `all()` or `with_track()`");
        }
        self.all = true;
        self
    }

    pub fn with_delay_mode(mut self, delay_mode: DelayMode) -> Self {
        self.delay_mode = Some(delay_mode);
        self
    }
}
