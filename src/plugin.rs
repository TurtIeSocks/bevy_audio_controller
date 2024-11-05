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
#[cfg(feature = "inspect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

use crate::{
    ac_assets::{load_assets, ACAssetLoader},
    channel::ChannelRegistration,
    global::GlobalChannel,
};

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ACAssetLoader>()
            .register_audio_channel::<GlobalChannel>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, assign_empty_sink_to_global);

        #[cfg(feature = "inspect")]
        app.register_type::<ACAssetLoader>();
    }
}

#[derive(Component, Debug)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
pub(super) struct HasChannel;

fn assign_empty_sink_to_global(
    mut commands: Commands,
    query: Query<(Entity, Option<&HasChannel>), Added<AudioSink>>,
) {
    for (entity, has_channel_opt) in query.iter() {
        if has_channel_opt.is_some() {
            commands.entity(entity).remove::<HasChannel>();
        } else {
            commands.entity(entity).insert(GlobalChannel);
        }
    }
}
