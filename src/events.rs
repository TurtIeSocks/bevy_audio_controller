use std::marker::PhantomData;

use bevy::{
    audio::PlaybackSettings,
    ecs::{component::Component, entity::Entity, event::Event},
};

use crate::{audio_files::AudioFiles, plugin::DelayMode};

#[derive(Event)]
pub struct PlayEvent<T: Component + Default> {
    pub(super) id: AudioFiles,
    pub(super) entity: Option<Entity>,
    pub(super) child: bool,
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) delay_mode: Option<DelayMode>,
    _marker: PhantomData<T>,
}

impl<T: Component + Default> PlayEvent<T> {
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

impl<Channel: Component + Default> From<AudioFiles> for PlayEvent<Channel> {
    fn from(id: AudioFiles) -> Self {
        Self::new(id)
    }
}

#[derive(Event)]
pub struct VolumeEvent<Channel: Component + Default> {
    pub(super) volume: f32,
    _marker: PhantomData<Channel>,
}

impl<Channel: Component + Default> VolumeEvent<Channel> {
    pub fn new(volume: f32) -> Self {
        Self {
            volume,
            _marker: PhantomData::<Channel>,
        }
    }
}

#[derive(Event)]
pub struct TrackEvent<Channel: Component + Default> {
    pub(super) id: Option<AudioFiles>,
    pub(super) settings: PlaybackSettings,
    _marker: PhantomData<Channel>,
}

impl<Channel: Component + Default> TrackEvent<Channel> {
    pub fn new(settings: PlaybackSettings) -> Self {
        Self {
            id: None,
            settings,
            _marker: PhantomData::<Channel>,
        }
    }

    pub fn with_track(mut self, id: AudioFiles) -> Self {
        self.id = Some(id);
        self
    }
}
