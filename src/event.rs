use std::marker::PhantomData;

use bevy::ecs::{entity::Entity, event::Event};

use crate::bounds::Bounds;

#[derive(Event)]
pub struct AudioControllerEvent<T: Bounds> {
    pub(super) id: String,
    pub(super) entity: Option<Entity>,
    pub(super) force: bool,
    _marker: PhantomData<T>,
}

impl<T: Bounds> AudioControllerEvent<T> {
    pub fn new<U: std::fmt::Display>(id: U) -> Self {
        Self {
            id: id.to_string(),
            entity: None,
            force: false,
            _marker: PhantomData::<T>,
        }
    }

    pub fn with_parent(self, entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..self
        }
    }

    pub fn with_force(mut self) -> Self {
        self.force = true;
        self
    }
}
