use crate::structs::vec3::Vec3;

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }
}
