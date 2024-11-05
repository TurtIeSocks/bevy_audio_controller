pub use bevy_audio_controller_derive::AudioChannel;

use crate::{
    audio_files::AudioFiles,
    bounds::ACBounds,
    events::{PlayEvent, SettingsEvent},
};

pub trait AudioChannel {
    fn play_event(id: AudioFiles) -> PlayEvent<Self>
    where
        Self: ACBounds;
    fn settings_event() -> SettingsEvent<Self>
    where
        Self: ACBounds;
}
