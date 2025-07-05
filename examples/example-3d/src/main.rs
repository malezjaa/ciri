use ciri::{
    create_bounding_box,
    engine::Engine,
    shapes::{create_cube, create_cylinder, create_sphere},
};
use ciri_math::{Transform, Vec3, from_glam_vec, vector};
use three_d::{
    Axes, ClearState, ColorMaterial, CpuMaterial, DirectionalLight, FrameOutput, Geometry,
};
use three_d_asset::Srgba;

fn main() {
    let mut engine = Engine::new().name("Builder example").build();

    {
        let sphere = create_sphere(
            &engine.context(),
            &CpuMaterial { albedo: Srgba::new(255, 0, 0, 200), ..Default::default() },
            Transform::from_translation(vector!(0.0, 1.3, 0.0)).scale(Vec3::splat(0.2)),
        );

        let cylinder = create_cylinder(
            &engine.context(),
            &CpuMaterial { albedo: Srgba::new(0, 255, 0, 200), ..Default::default() },
            Transform::from_translation(vector!(1.3, 0.0, 0.0)).scale(Vec3::splat(0.2)),
        );

        let cube = create_cube(
            &engine.context(),
            &CpuMaterial { albedo: Srgba::new(0, 0, 255, 200), ..Default::default() },
            Transform::from_translation(vector!(0.0, 0.0, 1.3)).scale(Vec3::splat(0.2)),
        );

        let axes = Axes::new(&engine.context(), 0.1, 2.0);

        let bounding_box_sphere = create_bounding_box(
            &engine.context(),
            sphere.aabb(),
            ColorMaterial { color: Srgba::BLACK, ..Default::default() },
        );

        let bounding_box_cube = create_bounding_box(
            &engine.context(),
            cube.aabb(),
            ColorMaterial { color: Srgba::BLACK, ..Default::default() },
        );

        let bounding_box_cylinder = create_bounding_box(
            &engine.context(),
            cylinder.aabb(),
            ColorMaterial { color: Srgba::BLACK, ..Default::default() },
        );

        let light0 = DirectionalLight::new(
            &engine.context(),
            1.0,
            Srgba::WHITE,
            from_glam_vec(vector!(0.0, -0.5, -0.5)),
        );
        let light1 = DirectionalLight::new(
            &engine.context(),
            1.0,
            Srgba::WHITE,
            from_glam_vec(vector!(0.0, 0.5, 0.5)),
        );
    }
}
