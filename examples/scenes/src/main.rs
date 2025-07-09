mod rotator;

use crate::rotator::Rotator;
use anyhow::Result;
use ciri::{
    engine::Engine,
    impl_scene,
    lights::{AmbientLight, DirectionalLight},
    logger::init_logger,
    math::Vec3,
    model::Model,
    scenes::{
        GameObject, ResultFuture, Scene, SceneAuto, SceneTrait, UpdateResult, components::Renderer,
    },
};
use ciri_math::{Transform, vector};
use log::error;
use three_d::{
    Context, CpuMaterial, CpuMesh, FrameInput, FrameOutput, Geometry, Gm, Mesh, PhysicalMaterial,
    Skybox, SurfaceSettings, Window, WindowSettings,
};
use three_d_asset::{
    Srgba, Texture2D,
    io::{RawAssets, load_async},
};

#[derive(Default)]
pub struct GameData {
    pub num: usize,
}

impl_scene!("Game", Game, GameData, (skybox, "examples/assets/environment.hdr" => Texture2D));
impl SceneTrait for Game {
    fn update(&mut self) -> UpdateResult {
        Ok(FrameOutput::default())
    }

    fn setup_sync(&mut self, ctx: Context) -> Result<()> {
        self.scene.setup_orbit_camera();

        let skybox = Skybox::new_from_equirectangular(&ctx, &self.skybox);
        self.scene.add_light(
            AmbientLight::builder()
                .color(Srgba::WHITE)
                .environment(&skybox.texture())
                .intensity(1.0)
                .build(&ctx),
        );

        self.scene.add_object(GameObject::new("environment").with_component(Renderer::new(skybox)));
        self.scene.add_object(GameObject::new("rotator").with_component(Rotator::new(0.5)));

        Ok(())
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
