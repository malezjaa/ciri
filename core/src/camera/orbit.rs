use crate::{camera::Camera, frame::Frame, structs::Vec3};
use three_d::{Event, MouseButton, Viewer};

#[derive(Clone, Copy, Debug)]
pub struct OrbitControl {
    pub target: Vec3,
    pub min_distance: f32,
    pub max_distance: f32,
}

impl OrbitControl {
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self { target, min_distance, max_distance }
    }

    pub fn handle_events(&mut self, camera: &mut Camera, frame: &mut Frame) -> bool {
        let mut change = false;
        let events = &mut frame.input.events;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion { delta, button, handled, .. } => {
                    if !*handled {
                        if Some(MouseButton::Left) == *button {
                            let speed = 0.01;
                            camera.inner.rotate_around_with_fixed_up(
                                self.target.into(),
                                speed * delta.0,
                                speed * delta.1,
                            );
                            *handled = true;
                            change = true;
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        let speed = 0.01 * self.target.distance(camera.position().into()) + 0.001;
                        camera.inner.zoom_towards(
                            self.target.into(),
                            speed * delta.1,
                            self.min_distance,
                            self.max_distance,
                        );
                        *handled = true;
                        change = true;
                    }
                }
                Event::PinchGesture { delta, handled, .. } => {
                    if !*handled {
                        let speed = self.target.distance(camera.position().into()) + 0.1;
                        camera.inner.zoom_towards(
                            self.target.into(),
                            speed * *delta,
                            self.min_distance,
                            self.max_distance,
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
