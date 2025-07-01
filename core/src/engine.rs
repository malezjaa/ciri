use crate::{
    camera::{
        Camera, CameraBuilder, CameraPresets,
        manager::{CameraId, CameraManager},
    },
    frame::Frame,
    options::EngineOptions,
};
use three_d::{Context, FrameOutput, Window};
use three_d_asset::Viewport;

pub struct Engine {
    window: Window,
    options: EngineOptions,
    camera_manager: CameraManager,
}

impl Engine {
    pub fn new(options: EngineOptions) -> Self {
        Self {
            options: options.clone(),
            window: Window::new(options.into()).unwrap(),
            camera_manager: CameraManager::new(),
        }
    }

    pub fn render_loop<F: 'static + FnMut(Frame) -> FrameOutput>(self, mut callback: F) {
        self.window.render_loop(move |input| callback(Frame::new(input)));
    }

    pub fn render_loop_with_camera<F: 'static + FnMut(&mut Frame, &Camera) -> FrameOutput>(
        mut self,
        mut callback: F,
    ) {
        if self.camera_manager.get_active_camera().is_none() {
            self.setup_default_camera();
        }

        self.window.render_loop(move |input| {
            let mut frame = Frame::new(input);

            self.camera_manager.handle_events(&mut frame);
            self.camera_manager.update_viewports(frame.viewport());

            if let Some(camera) = self.camera_manager.get_active_camera() {
                callback(&mut frame, camera)
            } else {
                FrameOutput::default()
            }
        });
    }

    pub fn viewport(&self) -> Viewport {
        self.window.viewport()
    }

    pub fn context(&self) -> Context {
        self.window.gl()
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        self.camera_manager.add_camera(camera)
    }

    pub fn remove_camera(&mut self, id: CameraId) -> Option<Camera> {
        self.camera_manager.remove_camera(id)
    }

    pub fn set_active_camera(&mut self, id: CameraId) -> bool {
        self.camera_manager.set_active_camera(id)
    }

    pub fn get_active_camera(&self) -> Option<&Camera> {
        self.camera_manager.get_active_camera()
    }

    pub fn get_active_camera_mut(&mut self) -> Option<&mut Camera> {
        self.camera_manager.get_active_camera_mut()
    }

    pub fn get_camera(&self, id: CameraId) -> Option<&Camera> {
        self.camera_manager.get_camera(id)
    }

    pub fn get_camera_mut(&mut self, id: CameraId) -> Option<&mut Camera> {
        self.camera_manager.get_camera_mut(id)
    }

    pub fn camera_ids(&self) -> impl Iterator<Item = CameraId> + '_ {
        self.camera_manager.camera_ids()
    }

    pub fn camera_manager(&self) -> &CameraManager {
        &self.camera_manager
    }

    pub fn camera_manager_mut(&mut self) -> &mut CameraManager {
        &mut self.camera_manager
    }

    pub fn camera(&self) -> CameraBuilder {
        CameraBuilder::new()
    }

    pub fn orbit_camera(&self) -> CameraBuilder {
        CameraPresets::orbit_around_origin()
    }

    pub fn camera_presets(&self) -> &CameraPresets {
        &CameraPresets
    }

    pub fn default_camera(&self) -> Camera {
        CameraPresets::orbit_around_origin().build(self.viewport())
    }

    pub fn setup_default_camera(&mut self) -> CameraId {
        let camera = self.default_camera();
        self.add_camera(camera)
    }

    pub fn setup_orbit_camera(&mut self) -> CameraId {
        let camera = self.orbit_camera().build(self.viewport());
        self.add_camera(camera)
    }

    pub fn setup_orbit_camera_at_distance(&mut self, distance: f32) -> CameraId {
        let camera =
            CameraPresets::orbit_around_origin_at_distance(distance).build(self.viewport());
        self.add_camera(camera)
    }

    pub fn setup_top_down_camera(&mut self, height: f32) -> CameraId {
        let camera = CameraPresets::top_down_at_height(height).build(self.viewport());
        self.add_camera(camera)
    }

    #[must_use]
    pub fn with_default_camera(mut self) -> Self {
        self.setup_default_camera();
        self
    }

    #[must_use]
    pub fn with_orbit_camera(mut self) -> Self {
        self.setup_orbit_camera();
        self
    }

    #[must_use]
    pub fn with_orbit_camera_at_distance(mut self, distance: f32) -> Self {
        self.setup_orbit_camera_at_distance(distance);
        self
    }

    #[must_use]
    pub fn with_top_down_camera(mut self, height: f32) -> Self {
        self.setup_top_down_camera(height);
        self
    }

    #[must_use]
    pub fn with_camera(mut self, builder: CameraBuilder) -> Self {
        self.add_camera(builder.build(self.viewport()));
        self
    }
}
