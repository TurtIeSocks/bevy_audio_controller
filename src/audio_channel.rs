pub use bevy_audio_controller_derive::AudioChannel;

use crate::{
    audio_files::AudioFiles,
    bounds::Bounds,
    events::{PlayEvent, SettingsEvent},
};

pub trait AudioChannel {
    fn play_event(id: AudioFiles) -> PlayEvent<Self>
    where
        Self: Bounds;
    fn settings_event() -> SettingsEvent<Self>
    where
        Self: Bounds;
}
