#[cfg(feature = "inspect")]
use bevy::reflect::{FromReflect, GetTypeRegistration, TypePath};
use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};

use super::channel::ChannelRegistration;

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_audio_channel::<GlobalAudioChannel>();
    }
}

#[derive(Default, Component)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub struct GlobalAudioChannel;
