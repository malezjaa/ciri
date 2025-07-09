use crate::scenes::components::Component;
use ciri_math::Transform;
use id_arena::Id;
use std::{any::TypeId, collections::HashMap, fmt::Debug};

pub type GameObjectId = Id<GameObject>;

pub struct GameObject {
    pub id: Option<GameObjectId>,
    pub name: String,
    pub transform: Transform,
    pub(crate) active: bool,
    pub(crate) components: HashMap<TypeId, Box<dyn Component>>,
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        Self {
            id: None,
            name: self.name.clone(),
            transform: self.transform,
            active: self.active,
            components: self.components.iter().map(|(k, v)| (*k, v.clone_component())).collect(),
        }
    }
}

impl Debug for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("GameObject \"{}\"", self.name))
            .field("active", &self.active)
            .field("transform", &self.transform)
            .field("components", &self.components)
            .finish()
    }
}

impl GameObject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            active: true,
            transform: Transform::identity(),
            components: HashMap::new(),
        }
    }

    pub fn new_temp(name: String) -> Self {
        Self {
            id: None,
            name,
            active: true,
            transform: Transform::identity(),
            components: HashMap::new(),
        }
    }

    pub fn enable(&mut self) {
        self.active = true;
    }

    pub fn disable(&mut self) {
        self.active = false;
    }
}
