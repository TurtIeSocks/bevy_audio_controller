use std::marker::PhantomData;

use bevy::{
    audio::PlaybackSettings,
    ecs::{component::Component, entity::Entity, event::Event},
};

use super::audio_files::AudioFiles;

#[derive(Event)]
pub struct PlayEvent<T: Component + Default> {
    pub(super) id: AudioFiles,
    pub(super) parent: Option<Entity>,
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) force: bool,
    // pub(super) handler: Option<Handle<AudioSource>>,
    _marker: PhantomData<T>,
}

impl<T: Component + Default> PlayEvent<T> {
    pub fn new(id: AudioFiles) -> Self {
        Self {
            id,
            parent: None,
            settings: None,
            force: false,
            // handler: None,
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

    // pub fn with_handler(mut self, handler: Handle<AudioSource>) -> Self {
    //     self.handler = Some(handler);
    //     self
    // }
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
