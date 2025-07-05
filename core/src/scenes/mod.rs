use three_d::HasContext;
pub mod components;
pub mod manager;

use crate::{
    camera::manager::CameraManager,
    engine::Engine,
    frame::Frame,
    lights::AbstractedLight,
    scenes::components::{Component, Renderer},
};
use ciri_math::Transform;
use id_arena::{Arena, Id};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use three_d::{ClearState, Context, FrameInput, FrameOutput, Light, Object, Viewer, Window};

pub type GameObjectId = Id<GameObject>;

pub struct Scene {
    pub name: &'static str,
    pub objects: HashMap<GameObjectId, GameObject>,
    pub(crate) camera_manager: CameraManager,
    pub id_arena: Arena<GameObject>,
    pub frame: Option<Frame>,
    pub lights: Vec<Box<dyn AbstractedLight>>,
}

impl Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {{ name: {} }}", self.name)
    }
}

pub struct GameObject {
    pub id: Option<GameObjectId>,
    pub name: String,
    pub transform: Transform,
    active: bool,
    parent: Option<GameObjectId>,
    children: Vec<GameObjectId>,
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        Self {
            id: None,
            name: self.name.clone(),
            transform: self.transform,
            active: self.active,
            parent: self.parent,
            children: self.children.clone(),
            components: self.components.iter().map(|(k, v)| (*k, v.clone_component())).collect(),
        }
    }
}

impl Debug for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameObject {{ name: {} }}", self.name)
    }
}

impl GameObject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            active: true,
            transform: Transform::identity(),
            parent: None,
            children: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn new_temp(name: String) -> Self {
        Self {
            id: None,
            name,
            active: true,
            transform: Transform::identity(),
            parent: None,
            children: Vec::new(),
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

    pub fn add_root_object(&mut self, temp: GameObject) -> GameObjectId {
        let id = self.id_arena.alloc_with_id(|id| GameObject {
            id: Some(id),
            name: temp.name,
            active: temp.active,
            transform: temp.transform,
            parent: None,
            children: Vec::new(),
            components: temp.components,
        });

        self.objects.insert(id, self.id_arena[id].clone());
        id
    }

    pub fn add_child_object(&mut self, parent_id: GameObjectId, temp: GameObject) -> GameObjectId {
        let id = self.id_arena.alloc_with_id(|id| GameObject {
            id: Some(id),
            name: temp.name,
            active: temp.active,
            transform: temp.transform,
            parent: Some(parent_id),
            children: Vec::new(),
            components: temp.components,
        });

        if let Some(parent) = self.id_arena.get_mut(parent_id) {
            parent.children.push(id);
        }

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

    pub fn reset(&mut self) {
        self.frame = None;
        self.objects.clear();
    }
}

pub trait SceneTrait: Any {
    fn full_update(&mut self, frame: &mut Frame) -> FrameOutput {
        let ctx = &frame.ctx;

        self.scene().frame = Some(frame.clone());
        self.update();

        let scene = self.scene();
        scene.camera_manager.handle_events(&mut frame.clone());
        
        let mut renderable = vec![];

        fn collect_active_objects(
            scene: &Scene,
            object_id: &GameObjectId,
            renderable: &mut Vec<Renderer>,
        ) {
            if let Some(object) = scene.objects().get(object_id) {
                if object.active {
                    if let Some(renderer) = object.get_component::<Renderer>() {
                        renderable.push(renderer.clone());
                    }

                    for child_id in &object.children {
                        collect_active_objects(scene, child_id, renderable);
                    }
                }
            }
        }

        for (id, object) in scene.objects() {
            if object.parent.is_none() {
                collect_active_objects(scene, id, &mut renderable);
            }
        }

        if let Some(camera) = scene.get_active_camera() {
            let mut objects: Vec<&dyn Object> = vec![];

            for renderer in &renderable {
                objects.push(&*renderer.to_render);
            }

            let built_lights: Vec<_> = scene.lights.iter().map(|l| l.build(ctx)).collect();
            let lights: Vec<&dyn Light> = built_lights.iter().map(|l| &**l).collect();

            frame.clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0));
            frame.render(camera, objects, lights.as_slice());
        }
        self.scene().reset();

        FrameOutput::default()
    }

    fn setup(&mut self) {}

    fn update(&mut self) -> FrameOutput;
    fn name(&self) -> &'static str;
    fn scene(&mut self) -> &mut Scene;
}

impl dyn SceneTrait {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[macro_export]
macro_rules! impl_scene {
    ($name:expr, $struct:ident, $data:ty) => {
        pub struct $struct {
            pub scene: Scene,
            pub data: $data,
        }

        impl Default for $struct {
            fn default() -> Self {
                Self { scene: Scene::new($name), data: <$data>::default() }
            }
        }

        impl $struct {
            pub fn build() -> Self {
                Self::default()
            }
        }
    };
}
