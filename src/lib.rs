use bevy::ecs::component::{Component, Immutable, StorageType};

use ac_traits::CommandAudioTracks;
use audio_files::AudioFiles;
use delay_mode::DelayMode;

mod audio_channel;
mod bounds;
mod channel;
mod delay_mode;
mod events;
mod global;
mod helpers;
mod plugin;
mod resources;

include!(concat!(env!("OUT_DIR"), "/audio_controller.rs"));

impl Component for AudioFiles {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn on_add() -> Option<bevy::ecs::component::ComponentHook> {
        Some(move |mut world, ctx| {
            let val: AudioFiles = world.get::<Self>(ctx.entity).unwrap().clone();
            #[cfg(feature = "log")]
            bevy::log::debug!("Adding audio track: {:?}", val);
            if world.get::<DelayMode>(ctx.entity).is_none() {
                world
                    .commands()
                    .entity(ctx.entity)
                    .insert(DelayMode::default())
                    .insert_audio_track(&val);
            }
        })
    }

    fn on_remove() -> Option<bevy::ecs::component::ComponentHook> {
        Some(move |mut world, ctx| {
            let val = world.get::<Self>(ctx.entity).unwrap().clone();
            #[cfg(feature = "log")]
            bevy::log::debug!("Removing audio track: {:?}", val);
            if world.get::<DelayMode>(ctx.entity).is_none() {
                world
                    .commands()
                    .entity(ctx.entity)
                    .remove::<DelayMode>()
                    .remove_audio_track(&val);
            }
        })
    }
}

pub mod prelude {
    pub use super::audio_channel::AudioChannel;
    pub use super::audio_files::AudioFiles;
    pub use super::bounds::ACBounds;
    pub use super::channel::*;
    pub use super::delay_mode::*;
    pub use super::events::*;
    pub use super::global::*;
    #[allow(unused)]
    pub use super::markers::*;
    pub use super::plugin::*;
    pub use super::resources::*;
}
