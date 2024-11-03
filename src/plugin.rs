#[cfg(feature = "inspect")]
use bevy::reflect::{FromReflect, GetTypeRegistration, TypePath};
use bevy::{
    app::{App, Plugin, Startup, Update},
    audio::AudioSink,
    ecs::{
        component::Component,
        entity::Entity,
        query::Added,
        system::{Commands, Query},
    },
};

use super::{
    channel::ChannelRegistration,
    handler_plugin::{load_assets, AssetLoader},
};

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoader>()
            .register_audio_channel::<GlobalAudioChannel>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, assign_to_global);
    }
}

#[derive(Default, Component)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub struct GlobalAudioChannel;

#[derive(Component)]
pub(super) struct NotGlobal;

fn assign_to_global(
    mut commands: Commands,
    query: Query<(Entity, Option<&NotGlobal>), Added<AudioSink>>,
) {
    for (entity, not_global) in query.iter() {
        if let Some(_) = not_global {
            commands.entity(entity).remove::<NotGlobal>();
        } else {
            commands.entity(entity).insert(GlobalAudioChannel);
        }
    }
}
