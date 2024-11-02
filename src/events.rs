use std::marker::PhantomData;

use bevy::{
    audio::PlaybackSettings,
    ecs::{entity::Entity, event::Event},
};

use super::{audio_files::AudioFiles, bounds::Bounds};

#[derive(Event)]
pub struct PlayEvent<T: Bounds> {
    pub(super) id: AudioFiles,
    pub(super) parent: Option<Entity>,
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) force: bool,
    _marker: PhantomData<T>,
}

impl<T: Bounds> PlayEvent<T> {
    pub fn new<U: std::fmt::Display>(id: U) -> Self {
        Self {
            id: id.to_string().into(),
            parent: None,
            settings: None,
            force: false,
            _marker: PhantomData::<T>,
        }
    }

    pub fn with_parent(self, entity: Entity) -> Self {
        Self {
            parent: Some(entity),
            ..self
        }
    }

    pub fn with_force(mut self) -> Self {
        self.force = true;
        self
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }
}

impl<Channel: Bounds> From<AudioFiles> for PlayEvent<Channel> {
    fn from(id: AudioFiles) -> Self {
        Self {
            id,
            parent: None,
            settings: None,
            force: false,
            _marker: PhantomData::<Channel>,
        }
    }
}

#[derive(Event)]
pub struct VolumeEvent<Channel: Bounds> {
    pub(super) volume: f32,
    _marker: PhantomData<Channel>,
}

impl<Channel: Bounds> VolumeEvent<Channel> {
    pub fn new(volume: f32) -> Self {
        Self {
            volume,
            _marker: PhantomData::<Channel>,
        }
    }
}

#[derive(Event)]
pub struct TrackEvent<Channel: Bounds> {
    pub(super) id: Option<AudioFiles>,
    pub(super) settings: PlaybackSettings,
    _marker: PhantomData<Channel>,
}

impl<Channel: Bounds> TrackEvent<Channel> {
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
