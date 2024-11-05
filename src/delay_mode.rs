use bevy::ecs::component::Component;

#[derive(Component, Default, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DelayMode {
    Immediate,
    #[default]
    Wait,
    Percent(i32),
    Milliseconds(i32),
}
