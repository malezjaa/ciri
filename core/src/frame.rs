use crate::scenes::Scene;
use three_d::{ClearState, Context, FrameInput, Light, Object, Viewer};
use three_d_asset::Viewport;

#[derive(Debug, Clone)]
pub struct Frame {
    pub input: FrameInput,
    pub ctx: Context,
}

impl Frame {
    pub fn new(input: FrameInput, ctx: Context) -> Self {
        Self { input, ctx }
    }

    pub fn clear(&self, state: ClearState) -> &Self {
        self.input.screen().clear(state);
        self
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
