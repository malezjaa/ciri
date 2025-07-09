use crate::scenes::Scene;
use three_d::{ClearState, Context, FrameInput, Light, Object, Viewer};
use three_d_asset::Viewport;

#[derive(Debug, Clone)]
pub struct Frame {
    pub input: FrameInput,
}

impl Frame {
    pub fn new(input: FrameInput) -> Self {
        Self { input }
    }

    pub fn clear(&self, state: ClearState) -> &Self {
        self.input.screen().clear(state);
        self
    }
    
    pub fn delta_time(&self) -> f32 {
        self.input.elapsed_time as f32
    }

    pub fn render(
        &self,
        camera: impl Viewer,
        objects: impl IntoIterator<Item = impl Object>,
        lights: &[&dyn Light],
    ) {
        self.input.screen().render(camera, objects, lights);
    }

    pub fn viewport(&self) -> Viewport {
        self.input.screen().viewport()
    }
}
