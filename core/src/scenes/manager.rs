use crate::scenes::{Scene, SceneTrait};
use anyhow::Result;
use futures::executor::block_on;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

pub struct SceneManager {
    scenes: HashMap<TypeId, Box<dyn SceneTrait>>,
    active: TypeId,
}

impl SceneManager {
    pub fn new() -> Self {
        Self { scenes: HashMap::new(), active: TypeId::of::<()>() }
    }

    pub fn register(&mut self, scene: impl SceneTrait + 'static) {
        self.scenes.insert(scene.type_id(), Box::new(scene));
    }

    pub fn get_mut_name<T: SceneTrait + 'static>(&mut self, name: String) -> Option<&mut T> {
        self.scenes
            .values_mut()
            .find(|scene| scene.name() == name)
            .map(|scene| scene.as_any_mut())?
            .downcast_mut::<T>()
    }

    pub fn get_mut<T: SceneTrait + 'static>(&mut self) -> Option<&mut T> {
        self.scenes
            .get_mut(&TypeId::of::<T>())
            .and_then(|scene| scene.as_any_mut().downcast_mut::<T>())
    }

    pub fn set_active<T: SceneTrait + 'static>(&mut self) -> Result<bool> {
        let type_id = TypeId::of::<T>();

        if !self.scenes.contains_key(&type_id) {
            return Ok(false);
        }

        if let Some(active) = self.active_scene_mut() {
            active.exit();
        }

        if let Some(scene) = self.scenes.get_mut(&type_id) {
            self.active = type_id;
            block_on(scene.setup_async())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn active_scene_mut(&mut self) -> Option<&mut dyn SceneTrait> {
        self.scenes.get_mut(&self.active).map(|s| s.as_mut())
    }

    pub fn is_registered<T: SceneTrait + 'static>(&self) -> bool {
        self.scenes.iter().any(|(_, scene)| scene.type_id() == TypeId::of::<T>())
    }

    pub fn is_active<T: SceneTrait + 'static>(&self) -> bool {
        self.active == TypeId::of::<T>()
    }
}
