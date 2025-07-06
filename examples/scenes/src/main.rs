use anyhow::Result;
use ciri::{
    engine::Engine,
    impl_scene,
    lights::DirectionalLight,
    logger::init_logger,
    math::Vec3,
    scenes::{GameObject, ResultFuture, Scene, SceneTrait, components::Renderer},
    shapes::create_cylinder,
};
use ciri_math::{Transform, vector};
use log::error;
use three_d::{
    CpuMaterial, CpuMesh, FrameInput, FrameOutput, Geometry, Gm, Mesh, PhysicalMaterial, Skybox,
    SurfaceSettings, Window, WindowSettings,
};
use three_d_asset::{
    Srgba,
    io::{RawAssets, load_async},
};
use ciri::scenes::UpdateResult;

#[derive(Default)]
pub struct GameData {
    pub num: usize,
    pub assets: RawAssets,
}

impl_scene!("Game", Game, GameData);
impl SceneTrait for Game {
    fn update_async(&mut self) -> ResultFuture<UpdateResult> {
        Box::pin(async move {
            let ctx = &self.scene.frame().ctx;

            // https://polyhaven.com/a/klippad_sunrise_2
            let mut loaded = load_async(&["examples/assets/sunrise_4k.hdr"]).await?;
            let skybox =
                Skybox::new_from_equirectangular(&ctx, &loaded.deserialize("sunrise_4k").unwrap());

            self.scene.add_root_object(
                GameObject::new("environment").with_component(Renderer::new(skybox)),
            );

            Ok(FrameOutput::default())
        })
    }

    fn setup_sync(&mut self) -> Result<()> {
        self.scene.setup_orbit_camera();
        self.scene.add_light(
            DirectionalLight::builder()
                .color(Srgba::WHITE)
                .direction(vector!(0.0, -0.5, -0.5))
                .intensity(1.0)
                .build(),
        );

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Game"
    }

    fn scene(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

fn main() -> anyhow::Result<()> {
    init_logger()?;
    let window = Window::new(WindowSettings {
        title: "Scenes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })?;
    let ctx = window.gl();

    let mut engine = Engine::new(ctx);

    engine.scenes.register(Game::build());
    engine.scenes.set_active::<Game>()?;

    window.render_loop(move |input| {
        engine.update(input).unwrap_or_else(|e| {
            error!("Error: {}", e);
            FrameOutput::default()
        })
    });

    Ok(())
}
