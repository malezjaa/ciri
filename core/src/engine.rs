use crate::{
    frame::Frame,
    scenes::{SceneTrait, UpdateResult, manager::SceneManager},
};
use three_d::{Context, FrameInput};

pub struct Engine {
    pub scenes: SceneManager,
}

impl Engine {
    fn update_inner(&mut self, input: FrameInput) -> UpdateResult {
        let scene = self
            .scenes
            .active_scene_mut()
            .expect("no active scene found. use engine.scenes.set_active::<T>() to set a scene");

        scene.full_update(&mut Frame::new(input))
    }

    pub fn update(&mut self, input: FrameInput) -> UpdateResult {
        self.update_inner(input)
    }

    pub fn new(context: Context) -> Self {
        Self { scenes: SceneManager::new(context.clone()) }
    }
}
