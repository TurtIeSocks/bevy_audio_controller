use bevy::ecs::component::Component;
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

use crate::{
    audio_channel::AudioChannel,
    events::{PlayEvent, SettingsEvent},
};

/// This is the global channel, any audio that is played without a channel will be automatically added to this channel
#[derive(Default, Component)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub struct GlobalChannel;

/// Type alias for the PlayEvent with the GlobalChannel
pub type GlobalPlayEvent = PlayEvent<GlobalChannel>;

/// Type alias for the SettingsEvent with the GlobalChannel
pub type GlobalSettingsEvent = SettingsEvent<GlobalChannel>;

impl AudioChannel for GlobalChannel {
    fn play_event(id: crate::audio_files::AudioFiles) -> PlayEvent<Self> {
        PlayEvent::new(id)
    }

    fn settings_event() -> SettingsEvent<Self> {
        SettingsEvent::new()
    }
}
