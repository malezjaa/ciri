use ciri_math::{Mat4, Vec3, from_glam_vec};
use three_d::{Context, DepthTexture2D, Environment, TextureCubeMap};
use three_d_asset::Srgba;

pub struct AmbientLightBuilder<'a> {
    pub intensity: f32,
    /// The base color of the light.
    pub color: Srgba,
    /// The light shining from the environment. This is calculated based on an environment map.
    pub environment: Option<&'a TextureCubeMap>,
}

impl<'a> AmbientLightBuilder<'a> {
    #[must_use]
    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    #[must_use]
    pub fn color(mut self, color: Srgba) -> Self {
        self.color = color;
        self
    }

    #[must_use]
    pub fn environment(mut self, environment: &'a TextureCubeMap) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn build(&self, context: &Context) -> three_d::AmbientLight {
        if let Some(environment) = self.environment {
            three_d::AmbientLight::new_with_environment(
                context,
                self.intensity,
                self.color,
                environment,
            )
        } else {
            three_d::AmbientLight::new(context, self.intensity, self.color)
        }
    }
}

pub struct AmbientLight<'a> {
    pub opts: AmbientLightBuilder<'a>,
}

impl<'a> Default for AmbientLightBuilder<'a> {
    fn default() -> Self {
        Self { intensity: 1.0, color: Srgba::WHITE, environment: None }
    }
}

impl<'a> AmbientLight<'a> {
    pub fn builder() -> AmbientLightBuilder<'a> {
        AmbientLightBuilder::default()
    }
}
