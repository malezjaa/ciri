use ciri::{
    engine::Engine,
    impl_scene,
    lights::DirectionalLight,
    math::Vec3,
    scenes::{GameObject, Scene, SceneTrait, components::Renderer},
    shapes::create_cylinder,
};
use ciri_math::{Transform, vector};
use three_d::{
    CpuMaterial, FrameInput, FrameOutput, Geometry, SurfaceSettings, Window, WindowSettings,
};
use three_d_asset::Srgba;

#[derive(Default)]
pub struct GameData {
    pub num: usize,
}

impl_scene!("Game", Game, GameData);
impl SceneTrait for Game {
    fn update_sync(&mut self) -> FrameOutput {
        let ctx = &self.scene.frame().ctx;

        let cylinder = create_cylinder(
            ctx,
            &CpuMaterial { albedo: Srgba::BLUE, ..Default::default() },
            Transform::from_translation(vector!(0.0, 0.0, 1.5)).scale(Vec3::splat(0.2)),
        );

        self.scene.add_root_object(
            GameObject::new("environment").with_component(Renderer::new(cylinder)),
        );

        FrameOutput::default()
    }

    fn setup(&mut self) {
        self.scene.setup_orbit_camera();
        self.scene.add_light(
            DirectionalLight::builder()
                .color(Srgba::WHITE)
                .direction(vector!(0.0, -0.5, -0.5))
                .intensity(1.0)
                .build(),
        );
    }

    fn name(&self) -> &'static str {
        "Game"
    }

    fn scene(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

fn main() -> anyhow::Result<()> {
    let window = Window::new(WindowSettings {
        title: "Scenes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })?;
    let ctx = window.gl();

    let mut engine = Engine::new(ctx);

    engine.scenes.register(Game::build());
    engine.scenes.set_active::<Game>();

    window.render_loop(move |input| engine.update(input));

    Ok(())
}
