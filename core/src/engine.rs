use futures::executor::block_on;
use crate::{
    frame::Frame,
    scenes::{SceneTrait, manager::SceneManager},
};
use three_d::{Context, FrameInput, FrameOutput};

pub struct Engine {
    pub context: Context,
    pub scenes: SceneManager,
}

impl Engine {
    async fn update_inner(&mut self, input: FrameInput) -> FrameOutput {
        let scene = self
            .scenes
            .active_scene_mut()
            .expect("no active scene found. use engine.scenes.set_active::<T>() to set a scene");

        scene.full_update(&mut Frame::new(input, self.context.clone())).await
    }
    
    pub fn update(&mut self, input: FrameInput) -> FrameOutput {
        block_on(self.update_inner(input))
    }
    
    pub fn new(context: Context) -> Self {
        Self { scenes: SceneManager::new(), context }
    }
}
