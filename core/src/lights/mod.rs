use crate::scenes::Scene;
use ciri_math::{Mat4, Vec3, from_glam_vec};
use three_d::{Context, DepthTexture2D, Light};
use three_d_asset::Srgba;

pub trait AbstractedLight {
    fn build(&self, context: &Context) -> Box<dyn Light>;
}

pub struct DirectionalLightBuilder {
    shadow_texture: Option<DepthTexture2D>,
    shadow_matrix: Mat4,
    /// The intensity of the light. This allows for higher intensity than 1 which can be used to simulate high intensity light sources like the sun.
    pub intensity: f32,
    /// The base color of the light.
    pub color: Srgba,
    /// The direction the light shines.
    pub direction: Vec3,
}

impl DirectionalLightBuilder {
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
    pub fn direction(mut self, direction: Vec3) -> Self {
        self.direction = direction;
        self
    }

    pub fn build(self) -> DirectionalLight {
        DirectionalLight { opts: self }
    }
}

pub struct DirectionalLight {
    pub opts: DirectionalLightBuilder,
}

impl Default for DirectionalLightBuilder {
    fn default() -> Self {
        Self {
            shadow_texture: None,
            shadow_matrix: Mat4::IDENTITY,
            intensity: 1.0,
            color: Srgba::WHITE,
            direction: Vec3::new(0.0, -1.0, 0.0),
        }
    }
}

impl DirectionalLight {
    pub fn builder() -> DirectionalLightBuilder {
        DirectionalLightBuilder::default()
    }

    pub fn build(&self, context: &Context) -> three_d::DirectionalLight {
        three_d::DirectionalLight::new(
            context,
            self.opts.intensity,
            self.opts.color,
            from_glam_vec(self.opts.direction),
        )
    }
}

impl AbstractedLight for DirectionalLight {
    fn build(&self, context: &Context) -> Box<dyn Light> {
        Box::new(self.build(context))
    }
}

impl Scene {
    pub fn add_light(&mut self, light: impl AbstractedLight + 'static) {
        self.lights.push(Box::new(light))
    }
}
