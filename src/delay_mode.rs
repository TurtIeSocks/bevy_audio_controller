#[cfg(feature = "inspect")]
use bevy::reflect::Reflect;
use bevy::{ecs::component::Component, log::warn};

/// Specifies how `bevy_audio_controller` should handle tracks on a per channel basis
#[derive(Component, Default, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
pub enum DelayMode {
    /// Waits for the track to finish before playing the next one
    ///
    /// This is the default behavior
    #[default]
    Wait,
    /// Always plays the track immediately
    Immediate,
    /// Waits for a percentage of the track to finish before playing the next one
    /// - Values between 0 and 100 will result in track overlapping
    /// - Values more than 100 will result in a gap between tracks
    ///
    /// Example:
    ///
    /// If you have a track that's 3 seconds, and you set this to 50, the next track will start playing 1.5 seconds after the current one starts playing
    Percent(u16),
    /// Waits for a specific number of milliseconds before playing the next one
    /// - Negative values will cause the next track to overlap with the current one
    /// - Positive values will cause a gap between tracks
    /// - This is relative to the track length
    ///
    /// Example:
    ///
    /// If you have a track that's 3 seconds, and you set this to -500, the next track will start playing 2.5 seconds after the current one starts playing
    Milliseconds(i16),
}

impl DelayMode {
    pub(super) fn get_delay(self, track_duration: f32) -> f32 {
        match self {
            DelayMode::Wait => track_duration,
            DelayMode::Immediate => 0.0,
            DelayMode::Percent(percent) => track_duration * (percent as f32 / 100.0),
            DelayMode::Milliseconds(ms) => {
                let ms: f32 = ms as f32 / 1000.0;
                if ms < -track_duration {
                    warn!("Delay ({}) should probably not be less than the negative length ({}) of the track. Use DelayMode::Immediate instead", ms, -track_duration);
                }
                track_duration + ms
            }
        }
        .max(0.0)
    }
}
