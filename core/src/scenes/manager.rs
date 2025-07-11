use crate::scenes::{Scene, SceneTrait};
use anyhow::Result;
use futures::executor::block_on;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};
use three_d::Context;
use three_d_asset::io::{load_and_deserialize_async, load_async};

pub struct SceneManager {
    scenes: HashMap<TypeId, Box<dyn SceneTrait>>,
    active: TypeId,
    context: Context,
}

impl SceneManager {
    pub fn new(context: Context) -> Self {
        Self { scenes: HashMap::new(), active: TypeId::of::<()>(), context }
    }

    /// Register a new scene, that you can later activate that with [SceneManager::set_active]
    pub fn register(&mut self, scene: impl SceneTrait + 'static) {
        self.scenes.insert(scene.type_id(), Box::new(scene));
    }

    /// Same as [SceneManager::get_mut], but gets the scene by name.
    pub fn get_mut_name<T: SceneTrait + 'static>(&mut self, name: String) -> Option<&mut T> {
        self.scenes
            .values_mut()
            .find(|scene| scene.name() == name)
            .map(|scene| scene.as_any_mut())?
            .downcast_mut::<T>()
    }

    /// Mutably gets the expected scene and casts it to `T`
    pub fn get_mut<T: SceneTrait + 'static>(&mut self) -> Option<&mut T> {
        self.scenes
            .get_mut(&TypeId::of::<T>())
            .and_then(|scene| scene.as_any_mut().downcast_mut::<T>())
    }

    /// Marks the provided type as an active scene.
    ///
    /// It loads the assets and invokes the setup method only once.
    ///
    /// Returns a boolean based if the switch was successful.
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

            if !scene.once_loaded() {
                block_on(async {
                    scene.load_assets().await?;
                    scene.setup_async(self.context.clone()).await?;
                    Ok::<_, anyhow::Error>(())
                })?;
            }

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
