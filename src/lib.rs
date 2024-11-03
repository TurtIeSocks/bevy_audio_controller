use ac_traits::CommandAudioTracks;

// mod bounds;
pub mod channel;
pub mod events;
pub mod plugin;
pub mod resources;

include!(concat!(env!("OUT_DIR"), "/audio_controller.rs"));

impl bevy::ecs::component::Component for audio_files::AudioFiles {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(_hooks: &mut bevy::ecs::component::ComponentHooks) {
        _hooks.on_add(|mut world, entity, _| {
            let val = world.get::<Self>(entity).unwrap().clone();
            if world.get::<plugin::DelayMode>(entity).is_none() {
                world
                    .commands()
                    .entity(entity)
                    .insert(plugin::DelayMode::default())
                    .insert_audio_track(&val);
            }
            if world.get::<bevy::core::Name>(entity).is_none() {
                world
                    .commands()
                    .entity(entity)
                    .insert(bevy::core::Name::new(val.to_string()));
            }
        });

        _hooks.on_remove(|mut world, entity, _| {
            let val = world.get::<Self>(entity).unwrap().clone();
            if world.get::<plugin::DelayMode>(entity).is_none() {
                world
                    .commands()
                    .entity(entity)
                    .remove::<plugin::DelayMode>()
                    .remove_audio_track(&val);
            }
        });
    }
}

pub mod prelude {
    pub use super::audio_files::AudioFiles;
    pub use super::channel::*;
    pub use super::events::*;
    pub use super::markers::*;
    pub use super::plugin::*;
    pub use super::resources::*;
}
