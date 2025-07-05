use crate::engine::Engine;
use ciri_math::{Transform, from_glam_mat4};
use three_d::{Context, CpuMaterial, CpuMesh, Geometry, Gm, Mesh, PhysicalMaterial};
use three_d_asset::Srgba;

pub fn create_object(
    context: &Context,
    mesh: CpuMesh,
    material: &CpuMaterial,
    transform: Transform,
) -> Gm<Mesh, PhysicalMaterial> {
    let mut object =
        Gm::new(Mesh::new(context, &mesh), PhysicalMaterial::new_transparent(context, material));
    object.set_transformation(from_glam_mat4(transform.to_matrix()));
    object
}
