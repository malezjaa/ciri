use three_d::HasContext;
pub mod components;
mod game_object;
pub use game_object::*;
pub mod manager;

pub use crate::{
    camera::manager::CameraManager,
    frame::Frame,
    scenes::{
        components::{Component, Renderer},
        game_object::{GameObject, GameObjectId},
    },
};
use anyhow::Result;
use ciri_math::Transform;
use id_arena::{Arena, Id};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{Debug, Pointer, format},
    pin::Pin,
    sync::Arc,
};
use three_d::{ClearState, Context, FrameInput, FrameOutput, Light, Object, Viewer, Window};
use three_d_asset::io::{RawAssets, load_and_deserialize_async};

pub struct Scene {
    pub name: &'static str,
    pub objects: HashMap<GameObjectId, GameObject>,
    pub(crate) camera_manager: CameraManager,
    pub id_arena: Arena<GameObject>,
    pub frame: Option<Frame>,
    pub lights: Vec<Arc<dyn Light + Send + Sync>>,
}

impl Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene").field("name", &self.name).field("objects", &self.objects).finish()
    }
}

impl Scene {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            objects: HashMap::new(),
            camera_manager: CameraManager::new(),
            id_arena: Arena::new(),
            frame: None,
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, temp: GameObject) -> GameObjectId {
        let id = self.id_arena.alloc_with_id(|id| GameObject {
            id: Some(id),
            name: temp.name,
            active: temp.active,
            transform: temp.transform,
            components: temp.components,
        });

        self.objects.insert(id, self.id_arena[id].clone());
        id
    }

    pub fn query(&self, id: GameObjectId) -> Option<&GameObject> {
        self.objects.get(&id)
    }

    pub fn frame(&self) -> &Frame {
        self.frame.as_ref().expect("frame should be set before update is called")
    }

    pub fn objects(&self) -> &HashMap<GameObjectId, GameObject> {
        &self.objects
    }
}

pub type UpdateResult = Result<FrameOutput>;
pub type ResultFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait SceneTrait: SceneAuto + Any + Send + Sync + 'static {
    fn update(&mut self) -> UpdateResult {
        Ok(FrameOutput::default())
    }

    fn full_update<'a>(&'a mut self, frame: &'a mut Frame) -> UpdateResult {
        {
            let scene = self.scene();
            scene.frame = Some(frame.clone());
            scene.camera_manager.handle_events(frame);
        }

        self.update()?;
        self.scene().update(frame.delta_time());

        let scene = self.scene();
        let mut objects = Vec::new();

        for (_, object) in scene.objects() {
            if !object.active {
                continue;
            }

            if let Some(renderer) = object.get_component::<Renderer>() {
                objects.push(&*renderer.to_render);
            }
        }

        if let Some(camera) = scene.get_active_camera() {
            frame.clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0));
            let light_refs: Vec<&dyn Light> =
                scene.lights.iter().map(|l| l.as_ref() as &dyn Light).collect();
            frame.render(camera, objects, &light_refs);
        }

        self.scene().frame = None;
        Ok(FrameOutput::default())
    }

    fn setup_async(&mut self, ctx: Context) -> ResultFuture<Result<()>> {
        Box::pin(async move { self.setup_sync(ctx) })
    }

    fn setup_sync(&mut self, _: Context) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self) {}
}

pub trait SceneAuto {
    fn name(&self) -> &'static str;
    fn scene(&mut self) -> &mut Scene;
    fn load_assets(&mut self) -> ResultFuture<Result<()>> {
        Box::pin(async move { Ok(()) })
    }
    fn once_loaded(&self) -> bool;
}

impl dyn SceneTrait {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[macro_export]
macro_rules! impl_scene {
    (
        $name:expr,
        $struct:ident,
        $data:ty
        $(, ( $( $asset_name:ident, $path:expr => $asset_ty:ty ),* $(,)? ) )?
    ) => {
        use three_d_asset::io::load_and_deserialize_async;

        pub struct $struct {
            pub scene: Scene,
            pub data: $data,
            pub once_loaded: bool,
             $(
                $(
                    pub $asset_name: $asset_ty,
                )*
            )?
        }

        impl Default for $struct {
            fn default() -> Self {
                Self {
                    scene: Scene::new($name),
                    data: <$data>::default(),
                    once_loaded: false,
                    $(
                        $(
                           $asset_name: <$asset_ty>::default(),
                        )*
                    )?
                }
            }
        }


        impl SceneAuto for $struct {
            fn name(&self) -> &'static str {
                $name
            }

            fn scene(&mut self) -> &mut Scene {
                &mut self.scene
            }

            fn load_assets(&mut self) -> ResultFuture<Result<()>> {
                Box::pin(async move {
                    $(
                        $(
                            self.$asset_name = load_and_deserialize_async($path).await?;
                        )*
                    )?

                    Ok(())
                })
            }

            fn once_loaded(&self) -> bool {
                self.once_loaded
            }
        }

        impl $struct {
            pub fn build() -> Self {
                Self::default()
            }
        }
    };
}
