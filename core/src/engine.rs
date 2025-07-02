use crate::{
    camera::{
        Camera, CameraBuilder, CameraPresets,
        manager::{CameraId, CameraManager},
    },
    frame::Frame,
    options::EngineOptions,
};
use three_d::{Context, FrameOutput, SurfaceSettings, Window, WindowSettings};
use three_d_asset::Viewport;

pub struct Engine {
    window: Option<Window>,
    options: EngineOptions,
    pub(crate) camera_manager: CameraManager,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            options: EngineOptions {
                window: WindowSettings {
                    title: "Engine".to_string(),
                    min_size: (100, 100),
                    max_size: None,
                    initial_size: None,
                    borderless: false,
                    surface_settings: SurfaceSettings::default(),
                },
            },
            window: None,
            camera_manager: CameraManager::new(),
        }
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref().expect("engine not initialized")
    }

    pub fn render_loop<F: 'static + FnMut(Frame) -> FrameOutput>(self, mut callback: F) {
        let window = self.window.expect("engine not initialized");
        window.render_loop(move |input| callback(Frame::new(input)));
    }

    pub fn render_loop_with_camera<F: 'static + FnMut(&mut Frame, &Camera) -> FrameOutput>(
        mut self,
        mut callback: F,
    ) {
        if self.camera_manager.get_active_camera().is_none() {
            self.setup_default_camera();
        }

        let window = self.window.expect("engine not initialized");
        window.render_loop(move |input| {
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
        self.window().viewport()
    }

    pub fn context(&self) -> Context {
        self.window().gl()
    }

    #[must_use]
    pub fn name(mut self, name: &str) -> Self {
        self.options.window.title = name.to_string();
        self
    }

    #[must_use]
    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.options.window.min_size = (width, height);
        self
    }

    #[must_use]
    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.options.window.max_size = Some((width, height));
        self
    }

    #[must_use]
    pub fn initial_size(mut self, width: u32, height: u32) -> Self {
        self.options.window.initial_size = Some((width, height));
        self
    }

    #[must_use]
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.options.window.borderless = borderless;
        self
    }

    #[must_use]
    pub fn surface_settings(mut self, settings: SurfaceSettings) -> Self {
        self.options.window.surface_settings = settings;
        self
    }

    pub fn build(mut self) -> Self {
        self.window = Some(Window::new(self.options.clone().into()).unwrap());
        self
    }
}
