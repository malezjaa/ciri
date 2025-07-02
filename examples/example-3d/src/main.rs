use ciri::{
    camera::{Camera, CameraPresets},
    create_bounding_box,
    engine::Engine,
    options::EngineOptions,
    shapes::{create_cube, create_cylinder, create_sphere},
};
use ciri_math::{Transform, Vec3, from_glam_vec, vector};
use three_d::{
    Axes, BoundingBox, ClearState, ColorMaterial, CpuMaterial, CpuMesh, DirectionalLight,
    FrameOutput, Geometry, Gm, Mesh, PhysicalMaterial,
};
use three_d_asset::{Mat4, Srgba};

fn main() {
    let engine = Engine::new(EngineOptions::builder().with_name("Builder Example").build())
        .with_orbit_camera();

    let mut sphere = create_sphere(
        &engine.context(),
        &CpuMaterial { albedo: Srgba { r: 255, g: 0, b: 0, a: 200 }, ..Default::default() },
        Transform::from_translation(vector!(0.0, 1.3, 0.0)).scale(Vec3::splat(0.2)),
    );

    let mut cylinder = create_cylinder(
        &engine.context(),
        &CpuMaterial { albedo: Srgba { r: 0, g: 255, b: 0, a: 200 }, ..Default::default() },
        Transform::from_translation(vector!(1.3, 0.0, 0.0)).scale(Vec3::splat(0.2)),
    );

    let mut cube = create_cube(
        &engine.context(),
        &CpuMaterial { albedo: Srgba { r: 0, g: 0, b: 255, a: 200 }, ..Default::default() },
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

    engine.render_loop_with_camera(move |mut frame, camera| {
        frame.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        frame.render(
            &camera,
            sphere
                .into_iter()
                .chain(&cylinder)
                .chain(&cube)
                .chain(&axes)
                .chain(&bounding_box_sphere)
                .chain(&bounding_box_cube)
                .chain(&bounding_box_cylinder),
            &[&light0, &light1],
        );

        FrameOutput::default()
    })
}
