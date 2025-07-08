mod material;

use crate::object::create_object;
use ciri_math::Transform;
use three_d::{Context, CpuMaterial, CpuMesh, Gm, Mesh, PhysicalMaterial};
use three_d_asset::KeyFrameAnimation;

pub struct Model;

impl<'a> Model {
    pub fn builder() -> ModelBuilder<'a> {
        ModelBuilder {
            mesh: None,
            material: None,
            transform: Transform::identity(),
            animation: None,
        }
    }
}

struct ModelBuilder<'a> {
    mesh: Option<CpuMesh>,
    material: Option<&'a CpuMaterial>,
    transform: Transform,
    animation: Option<KeyFrameAnimation>,
}

impl<'a> ModelBuilder<'a> {
    #[must_use]
    pub fn mesh(mut self, mesh: CpuMesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    #[must_use]
    pub fn material(mut self, material: &'a CpuMaterial) -> Self {
        self.material = Some(material);
        self
    }

    #[must_use]
    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(self, ctx: &Context) -> Gm<Mesh, PhysicalMaterial> {
        let mesh = self.mesh.expect("Mesh is required");
        let material = self.material.expect("Material is required");
        create_object(ctx, mesh, material, self.transform)
    }
}
