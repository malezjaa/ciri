use crate::{camera::Camera, frame::Frame};
use std::collections::HashMap;
use three_d_asset::Viewport;

pub type CameraId = u32;

pub struct CameraManager {
    cameras: HashMap<CameraId, Camera>,
    active_camera: Option<CameraId>,
    next_id: CameraId,
}

impl Default for CameraManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraManager {
    pub fn new() -> Self {
        Self { cameras: HashMap::new(), active_camera: None, next_id: 0 }
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        let id = self.next_id;
        self.next_id += 1;
        self.cameras.insert(id, camera);

        if self.active_camera.is_none() {
            self.active_camera = Some(id);
        }

        id
    }

    pub fn remove_camera(&mut self, id: CameraId) -> Option<Camera> {
        let camera = self.cameras.remove(&id);

        if self.active_camera == Some(id) {
            self.active_camera = self.cameras.keys().next().copied();
        }

        camera
    }

    pub fn set_active_camera(&mut self, id: CameraId) -> bool {
        if self.cameras.contains_key(&id) {
            self.active_camera = Some(id);
            true
        } else {
            false
        }
    }

    pub fn get_active_camera(&self) -> Option<&Camera> {
        self.active_camera.and_then(|id| self.cameras.get(&id))
    }

    pub fn get_active_camera_mut(&mut self) -> Option<&mut Camera> {
        self.active_camera.and_then(|id| self.cameras.get_mut(&id))
    }

    pub fn get_camera(&self, id: CameraId) -> Option<&Camera> {
        self.cameras.get(&id)
    }

    pub fn get_camera_mut(&mut self, id: CameraId) -> Option<&mut Camera> {
        self.cameras.get_mut(&id)
    }

    pub fn camera_ids(&self) -> impl Iterator<Item = CameraId> + '_ {
        self.cameras.keys().copied()
    }

    pub fn handle_events(&mut self, frame: &mut Frame) -> bool {
        if let Some(camera) = self.get_active_camera_mut() {
            if camera.auto_viewport {
                camera.set_viewport(frame.viewport());
            }
            camera.handle_events(frame)
        } else {
            false
        }
    }

    pub fn update_viewports(&mut self, viewport: Viewport) {
        for camera in self.cameras.values_mut() {
            if camera.auto_viewport {
                camera.set_viewport(viewport);
            }
        }
    }
}
