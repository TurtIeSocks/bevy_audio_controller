#[cfg(feature = "inspect")]
use bevy::reflect::Reflect;
use bevy::{ecs::component::Component, log::warn};

#[derive(Component, Default, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
pub enum DelayMode {
    #[default]
    Wait,
    Immediate,
    Percent(u16),
    Milliseconds(i16),
}

impl DelayMode {
    pub fn get_delay(self, track_duration: f32) -> f32 {
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
