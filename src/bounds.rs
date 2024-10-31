use bevy::ecs::component::Component;
#[cfg(feature = "inspect")]
use bevy::reflect::{FromReflect, GetTypeRegistration, TypePath};

#[cfg(not(feature = "inspect"))]
pub trait Bounds: Component + Default {}

#[cfg(not(feature = "inspect"))]
impl<T> Bounds for T where T: Component + Default {}

#[cfg(feature = "inspect")]
pub trait Bounds: Component + Default + GetTypeRegistration + FromReflect + TypePath {}

#[cfg(feature = "inspect")]
impl<T> Bounds for T where T: Component + Default + GetTypeRegistration + FromReflect + TypePath {}
