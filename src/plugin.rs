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

use crate::prelude::AudioFiles;

use super::{
    ac_assets::{load_assets, AssetLoader},
    channel::ChannelRegistration,
};

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoader>()
            .register_audio_channel::<GlobalAudioChannel>()
            .add_systems(Startup, load_assets)
            .add_systems(
                Update,
                (assign_rogue_sink_to_global, assign_rogue_audio_to_global),
            );
    }
}

#[derive(Default, Component)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub struct GlobalAudioChannel;

#[derive(Component)]
pub(super) struct HasChannel;

#[derive(Component, Default, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DelayMode {
    #[default]
    Immediate,
    Wait,
    Percent(i32),
    Milliseconds(i32),
}

fn assign_rogue_sink_to_global(
    mut commands: Commands,
    query: Query<(Entity, Option<&HasChannel>), Added<AudioSink>>,
) {
    for (entity, has_channel) in query.iter() {
        if has_channel.is_some() {
            commands.entity(entity).remove::<HasChannel>();
        } else {
            commands.entity(entity).insert(GlobalAudioChannel);
        }
    }
}

fn assign_rogue_audio_to_global(
    mut commands: Commands,
    query: Query<(Entity, Option<&HasChannel>), Added<AudioFiles>>,
) {
    for (entity, not_global) in query.iter() {
        if not_global.is_some() {
            commands.entity(entity).remove::<HasChannel>();
        } else {
            commands.entity(entity).insert(GlobalAudioChannel);
        }
    }
}
