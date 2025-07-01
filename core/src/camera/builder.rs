use crate::{
    camera::{Camera, CameraType, ControlType},
    structs::{Transform, Vec3},
    vector,
};
use three_d_asset::{Viewport, degrees};

pub struct CameraBuilder {
    camera_type: CameraType,
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    near: f32,
    far: f32,
    control: ControlType,
    auto_viewport: bool,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            camera_type: CameraType::Perspective,
            position: vector!(5.0, 2.0, 5.0),
            target: Vec3::ZERO,
            up: vector!(0.0, 1.0, 0.0),
            fov: 45.0,
            near: 0.1,
            far: 1000.0,
            control: ControlType::None,
            auto_viewport: true,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn perspective(mut self) -> Self {
        self.camera_type = CameraType::Perspective;
        self
    }

    #[must_use]
    pub fn orthographic(mut self) -> Self {
        self.camera_type = CameraType::Orthographic;
        self
    }

    #[must_use]
    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = vector!(x, y, z);
        self
    }
    
    #[must_use]
    pub fn position_vec(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn look_at(mut self, x: f32, y: f32, z: f32) -> Self {
        self.target = vector!(x, y, z);
        self
    }

    #[must_use]
    pub fn target(mut self, target: Vec3) -> Self {
        self.target = target;
        self
    }

    #[must_use]
    pub fn target_origin(mut self) -> Self {
        self.target = Vec3::ZERO;
        self
    }

    #[must_use]
    pub fn up(mut self, x: f32, y: f32, z: f32) -> Self {
        self.up = vector!(x, y, z);
        self
    }

    #[must_use]
    pub fn up_vec(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    #[must_use]
    pub fn fov(mut self, degrees: f32) -> Self {
        self.fov = degrees;
        self
    }

    #[must_use]
    pub fn near_far(mut self, near: f32, far: f32) -> Self {
        self.near = near;
        self.far = far;
        self
    }

    #[must_use]
    pub fn near(mut self, near: f32) -> Self {
        self.near = near;
        self
    }

    #[must_use]
    pub fn far(mut self, far: f32) -> Self {
        self.far = far;
        self
    }

    #[must_use]
    pub fn with_orbit_controls(mut self) -> Self {
        self.control = ControlType::Orbit { min_distance: 1.0, max_distance: 100.0 };
        self
    }

    #[must_use]
    pub fn with_orbit_controls_limits(mut self, min_distance: f32, max_distance: f32) -> Self {
        self.control = ControlType::Orbit { min_distance, max_distance };
        self
    }

    #[must_use]
    pub fn no_controls(mut self) -> Self {
        self.control = ControlType::None;
        self
    }

    #[must_use]
    pub fn auto_viewport(mut self, auto: bool) -> Self {
        self.auto_viewport = auto;
        self
    }

    pub fn build(self, viewport: Viewport) -> Camera {
        let transform = Transform::new(self.position, self.target - self.position, Vec3::ZERO);

        let mut camera = match self.camera_type {
            CameraType::Perspective => Camera::new_3d(
                viewport,
                &transform,
                self.up,
                degrees(self.fov),
                self.near,
                self.far,
            ),
            CameraType::Orthographic => Camera::new_2d(viewport),
        };

        camera.control = self.control;
        camera.target = self.target;
        camera.auto_viewport = self.auto_viewport;

        camera
    }
}
