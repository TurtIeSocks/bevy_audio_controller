use bevy::{
    core::Name,
    ecs::component::{Component, ComponentHooks, StorageType},
    log::debug,
};

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

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, entity, _| {
            let val: AudioFiles = world.get::<Self>(entity).unwrap().clone();
            debug!("Adding audio track: {:?}", val);
            if world.get::<DelayMode>(entity).is_none() {
                world
                    .commands()
                    .entity(entity)
                    .insert(DelayMode::default())
                    .insert_audio_track(&val);
            }
            if world.get::<Name>(entity).is_none() {
                world
                    .commands()
                    .entity(entity)
                    .insert(Name::new(val.to_string()));
            }
        });

        _hooks.on_remove(|mut world, entity, _| {
            let val = world.get::<Self>(entity).unwrap().clone();
            debug!("Removing audio track: {:?}", val);
            if world.get::<DelayMode>(entity).is_none() {
                world
                    .commands()
                    .entity(entity)
                    .remove::<DelayMode>()
                    .remove_audio_track(&val);
            }
        });
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
