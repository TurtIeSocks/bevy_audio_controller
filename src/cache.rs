use std::marker::PhantomData;

#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};
use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
    utils::hashbrown::HashMap,
};

use crate::bounds::Bounds;

include!(concat!(env!("OUT_DIR"), "/audio_lengths.rs"));

#[derive(Default, Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Resource))]
pub(super) struct AudioCache<T: Bounds> {
    pub(super) default_time: f32,
    pub(super) map: HashMap<String, Timer>,
    #[cfg_attr(feature = "inspect", reflect(ignore))]
    _marker: PhantomData<T>,
}

impl<T: Bounds> AudioCache<T> {
    pub(super) fn new(default_time: f32) -> Self {
        Self {
            default_time,
            ..Default::default()
        }
    }

    pub(super) fn can_play_sfx(&self, id: &str) -> bool {
        self.map
            .get(&id.to_string())
            .map_or(true, |timer| timer.finished())
    }

    pub(super) fn reset_entry(&mut self, id: String) {
        let duration = get_audio_file_length(&id).unwrap_or(self.default_time);
        self.map
            .entry(id)
            .or_insert(Timer::from_seconds(duration, TimerMode::Once))
            .reset()
    }
}
