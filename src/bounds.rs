use bevy::ecs::component::Component;
#[cfg(feature = "inspect")]
use bevy::reflect::{FromReflect, GetTypeRegistration, TypePath};

/// A trait that `bevy_audio_controller` uses internally to restrict generics.
///
/// If you want to use `bevy_audio_controller` with `bevy-inspector-egui`, you need to enable the `inspect` feature to derive `Reflect` for your components & resources.
#[cfg(not(feature = "inspect"))]
pub trait ACBounds: Component + Default {}

#[cfg(not(feature = "inspect"))]
impl<T> ACBounds for T where T: Component + Default {}

/// A trait that `bevy_audio_controller` uses internally to restrict generics.
#[cfg(feature = "inspect")]
pub trait ACBounds: Component + Default + GetTypeRegistration + FromReflect + TypePath {}

#[cfg(feature = "inspect")]
impl<T> ACBounds for T where T: Component + Default + GetTypeRegistration + FromReflect + TypePath {}
