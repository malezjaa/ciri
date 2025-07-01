use ciri_core::{
    camera::{Camera, CameraPresets, OrbitControl},
    engine::Engine,
    math::degrees,
    options::EngineOptions,
    structs::{Transform, Vec3},
    vector,
};
use three_d::{
    Axes, BoundingBox, ClearState, ColorMaterial, CpuMaterial, CpuMesh, DirectionalLight,
    FrameOutput, Geometry, Gm, Mesh, PhysicalMaterial,
};
use three_d_asset::{Mat4, Srgba};

fn main() {
    let engine = Engine::new(EngineOptions::builder().with_name("Builder Example").build())
        .with_camera(
            CameraPresets::orbit_around_origin_at_distance(8.0).fov(60.0).near_far(0.1, 500.0),
        );

    let mut sphere = Gm::new(
        Mesh::new(&engine.context(), &CpuMesh::sphere(16)),
        PhysicalMaterial::new_transparent(
            &engine.context(),
            &CpuMaterial { albedo: Srgba { r: 255, g: 0, b: 0, a: 200 }, ..Default::default() },
        ),
    );
    sphere.set_transformation(
        Mat4::from_translation(vector!(0.0, 1.3, 0.0).into()) * Mat4::from_scale(0.2),
    );
    let mut cylinder = Gm::new(
        Mesh::new(&engine.context(), &CpuMesh::cylinder(16)),
        PhysicalMaterial::new_transparent(
            &engine.context(),
            &CpuMaterial { albedo: Srgba { r: 0, g: 255, b: 0, a: 200 }, ..Default::default() },
        ),
    );
    cylinder.set_transformation(
        Mat4::from_translation(vector!(1.3, 0.0, 0.0).into()) * Mat4::from_scale(0.2),
    );
    let mut cube = Gm::new(
        Mesh::new(&engine.context(), &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &engine.context(),
            &CpuMaterial { albedo: Srgba { r: 0, g: 0, b: 255, a: 100 }, ..Default::default() },
        ),
    );
    cube.set_transformation(
        Mat4::from_translation(vector!(0.0, 0.0, 1.3).into()) * Mat4::from_scale(0.2),
    );
    let axes = Axes::new(&engine.context(), 0.1, 2.0);
    let bounding_box_sphere = Gm::new(
        BoundingBox::new(&engine.context(), sphere.aabb()),
        ColorMaterial { color: Srgba::BLACK, ..Default::default() },
    );
    let bounding_box_cube = Gm::new(
        BoundingBox::new(&engine.context(), cube.aabb()),
        ColorMaterial { color: Srgba::BLACK, ..Default::default() },
    );
    let bounding_box_cylinder = Gm::new(
        BoundingBox::new(&engine.context(), cylinder.aabb()),
        ColorMaterial { color: Srgba::BLACK, ..Default::default() },
    );

    let light0 = DirectionalLight::new(
        &engine.context(),
        1.0,
        Srgba::WHITE,
        vector!(0.0, -0.5, -0.5).into(),
    );
    let light1 =
        DirectionalLight::new(&engine.context(), 1.0, Srgba::WHITE, vector!(0.0, 0.5, 0.5).into());

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
