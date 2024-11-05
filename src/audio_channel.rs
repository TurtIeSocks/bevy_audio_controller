pub use bevy_audio_controller_derive::AudioChannel;

use crate::{audio_files, events};

pub trait AudioChannel {
    fn play_event(id: audio_files::AudioFiles) -> events::PlayEvent<Self>
    where
        Self: bevy::ecs::component::Component + Default;
    fn settings_event() -> events::SettingsEvent<Self>
    where
        Self: bevy::ecs::component::Component + Default;
}
