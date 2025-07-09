use crate::scenes::game_object::GameObject;
use std::{any::TypeId, collections::HashSet};

pub trait Updateable {
    fn update(&mut self, delta_time: f32, game_object: &mut GameObject);
    fn update_priority(&self) -> i32 {
        0
    }
}

#[derive(Default)]
pub struct ComponentRequirements {
    pub required: HashSet<TypeId>,
    pub conflicts: HashSet<TypeId>,
}

pub trait ComponentLifecycle {
    fn on_add(&mut self) {}
    fn on_remove(&mut self) {}
    fn get_requirements(&self) -> ComponentRequirements {
        ComponentRequirements::default()
    }
}
