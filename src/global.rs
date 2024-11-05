use bevy::ecs::component::Component;
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

use crate::{
    audio_channel::AudioChannel,
    events::{PlayEvent, SettingsEvent},
};

#[derive(Default, Component)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub struct GlobalChannel;

pub type GlobalPlayEvent = PlayEvent<GlobalChannel>;
pub type GlobalSettingsEvent = SettingsEvent<GlobalChannel>;

impl AudioChannel for GlobalChannel {
    fn play_event(id: crate::audio_files::AudioFiles) -> PlayEvent<Self> {
        PlayEvent::new(id)
    }

    fn settings_event() -> SettingsEvent<Self> {
        SettingsEvent::new()
    }
}
