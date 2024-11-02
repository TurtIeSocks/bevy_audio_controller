#[cfg(feature = "inspect")]
use bevy::reflect::{FromReflect, GetTypeRegistration, TypePath};
use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};

use super::{channel::ChannelRegistration, handler_plugin::HandlerPlugin};

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HandlerPlugin)
            .register_audio_channel::<GlobalAudioChannel>();
    }
}

#[derive(Default, Component)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub struct GlobalAudioChannel;
