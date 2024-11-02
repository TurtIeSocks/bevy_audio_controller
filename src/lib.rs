mod bounds;
pub mod channel;
pub mod events;
pub mod plugin;
pub mod resources;

include!(concat!(env!("OUT_DIR"), "/audio_controller.rs"));

pub mod prelude {
    pub use super::ac_traits::InsertAudioTrack;
    pub use super::audio_files::AudioFiles;
    pub use super::channel::*;
    pub use super::events::*;
    pub use super::markers::*;
    pub use super::plugin::*;
    pub use super::resources::*;
}
