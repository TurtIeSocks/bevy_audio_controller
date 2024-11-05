use bevy::ecs::component::Component;
#[cfg(feature = "inspect")]
use bevy::reflect::Reflect;

#[derive(Component, Default, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
pub enum DelayMode {
    #[default]
    Wait,
    Immediate,
    // Percent(i32),
    // Milliseconds(i32),
}
