use crate::object::create_object;
use ciri_math::Transform;
use three_d::{Context, CpuMaterial, CpuMesh, Gm, Mesh, PhysicalMaterial};

pub fn create_sphere(
    ctx: &Context,
    material: &CpuMaterial,
    transform: Transform,
) -> Gm<Mesh, PhysicalMaterial> {
    create_object(ctx, CpuMesh::sphere(16), material, transform)
}

pub fn create_cylinder(
    ctx: &Context,
    material: &CpuMaterial,
    transform: Transform,
) -> Gm<Mesh, PhysicalMaterial> {
    create_object(ctx, CpuMesh::cylinder(16), material, transform)
}

pub fn create_cube(
    ctx: &Context,
    material: &CpuMaterial,
    transform: Transform,
) -> Gm<Mesh, PhysicalMaterial> {
    create_object(ctx, CpuMesh::cube(), material, transform)
}
