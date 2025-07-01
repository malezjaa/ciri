use std::f32::consts::FRAC_1_SQRT_2;
use crate::camera::CameraBuilder;

pub struct CameraPresets;

impl CameraPresets {
    pub fn orbit_around_origin() -> CameraBuilder {
        CameraBuilder::new()
            .perspective()
            .position(8.0, 4.0, 8.0)
            .target_origin()
            .with_orbit_controls()
    }

    pub fn orbit_around_origin_at_distance(distance: f32) -> CameraBuilder {
        let pos = distance * FRAC_1_SQRT_2;
        CameraBuilder::new()
            .perspective()
            .position(pos, pos * 0.5, pos)
            .target_origin()
            .with_orbit_controls()
    }

    pub fn top_down() -> CameraBuilder {
        CameraBuilder::new()
            .orthographic()
            .position(0.0, 10.0, 0.0)
            .target_origin()
            .up(0.0, 0.0, -1.0)
    }

    pub fn top_down_at_height(height: f32) -> CameraBuilder {
        CameraBuilder::new()
            .orthographic()
            .position(0.0, height, 0.0)
            .target_origin()
            .up(0.0, 0.0, -1.0)
    }

    pub fn side_view() -> CameraBuilder {
        CameraBuilder::new()
            .perspective()
            .position(10.0, 0.0, 0.0)
            .target_origin()
            .with_orbit_controls()
    }

    pub fn front_view() -> CameraBuilder {
        CameraBuilder::new()
            .perspective()
            .position(0.0, 0.0, 10.0)
            .target_origin()
            .with_orbit_controls()
    }

    pub fn isometric() -> CameraBuilder {
        CameraBuilder::new()
            .orthographic()
            .position(5.0, 5.0, 5.0)
            .target_origin()
            .with_orbit_controls()
    }
}
