mod builder;
mod orbit;
mod presets;
pub mod manager;

use crate::{
    structs::{Transform, Vec3},
    *,
};
use three_d::{ColorMapping, Event, Mat4, Radians, ToneMapping, Viewer, Viewport};

pub use orbit::*;
pub use builder::*;
pub use presets::*;
use crate::frame::Frame;

#[derive(Debug, Clone, Copy)]
pub enum CameraType {
    Perspective,
    Orthographic,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlType {
    None,
    Orbit { min_distance: f32, max_distance: f32 },
}

pub struct Camera {
    pub inner: three_d_asset::Camera,
    pub tone_mapping: ToneMapping,
    pub color_mapping: ColorMapping,
    pub control: ControlType,
    pub target: Vec3,
    pub auto_viewport: bool,
}

impl Viewer for Camera {
    fn position(&self) -> three_d_asset::Vec3 {
        self.inner.position()
    }

    fn view(&self) -> Mat4 {
        self.inner.view()
    }

    fn projection(&self) -> Mat4 {
        self.inner.projection()
    }

    fn viewport(&self) -> Viewport {
        self.inner.viewport()
    }

    fn z_near(&self) -> f32 {
        self.inner.z_near()
    }

    fn z_far(&self) -> f32 {
        self.inner.z_far()
    }

    fn color_mapping(&self) -> ColorMapping {
        self.color_mapping
    }

    fn tone_mapping(&self) -> ToneMapping {
        self.tone_mapping
    }
}

impl Camera {
    pub fn new_2d(viewport: Viewport) -> Self {
        let center = vector!(viewport.width as f32 / 2.0, viewport.height as f32 / 2.0, 1.0);
        let target = center - vector!(0.0, 0.0, 1.0);

        Self {
            inner: three_d_asset::Camera::new_orthographic(
                viewport,
                center.into(),
                target.into(),
                vector!(0.0, 1.0, 0.0).into(),
                viewport.height as f32,
                0.0,
                10.0,
            ),
            tone_mapping: ToneMapping::default(),
            color_mapping: ColorMapping::default(),
            control: ControlType::None,
            target: target.into(),
            auto_viewport: true,
        }
    }

    pub fn new_3d(
        viewport: Viewport,
        transform: &Transform,
        up: Vec3,
        fov_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        let target = transform.position + transform.rotation;

        Self {
            inner: three_d_asset::Camera::new_perspective(
                viewport,
                transform.position.into(),
                target.into(),
                up.into(),
                fov_y,
                z_near,
                z_far,
            ),
            tone_mapping: ToneMapping::default(),
            color_mapping: ColorMapping::default(),
            control: ControlType::None,
            target,
            auto_viewport: true,
        }
    }

    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn set_viewport(&mut self, viewport: Viewport) -> bool {
        self.inner.set_viewport(viewport)
    }

    pub fn handle_events(&mut self, frame: &mut Frame) -> bool {
        match self.control {
            ControlType::Orbit { min_distance, max_distance } => {
                self.handle_orbit_events(frame, min_distance, max_distance)
            }
            ControlType::None => false,
        }
    }

    fn handle_orbit_events(&mut self, frame: &mut Frame, min_distance: f32, max_distance: f32) -> bool {
        let mut change = false;
        let events = &mut frame.input.events;

        for event in events.iter_mut() {
            match event {
                Event::MouseMotion { delta, button, handled, .. } => {
                    if !*handled && button == &Some(three_d::MouseButton::Left) {
                        let speed = 0.01;
                        self.inner.rotate_around_with_fixed_up(
                            self.target.into(),
                            speed * delta.0,
                            speed * delta.1,
                        );
                        *handled = true;
                        change = true;
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        let speed = 0.01 * self.target.distance(self.position().into()) + 0.001;
                        self.inner.zoom_towards(
                            self.target.into(),
                            speed * delta.1,
                            min_distance,
                            max_distance,
                        );
                        *handled = true;
                        change = true;
                    }
                }
                Event::PinchGesture { delta, handled, .. } => {
                    if !*handled {
                        let speed = self.target.distance(self.position().into()) + 0.1;
                        self.inner.zoom_towards(
                            self.target.into(),
                            speed * *delta,
                            min_distance,
                            max_distance,
                        );
                        *handled = true;
                        change = true;
                    }
                }
                _ => {}
            }
        }
        change
    }
}

