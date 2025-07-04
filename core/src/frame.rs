use three_d::{ClearState, FrameInput, Light, Object, Viewer};
use three_d_asset::Viewport;

#[derive(Debug)]
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
