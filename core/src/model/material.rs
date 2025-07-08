use three_d::{Context, PhysicalMaterial};
use three_d_asset::{LightingModel, PbrMaterial, Srgba, Texture2D};

pub struct Material;

impl Material {
    pub fn builder() -> MaterialBuilder {
        MaterialBuilder {
            material_type: None,
            name: "default".to_string(),
            albedo: Srgba::WHITE,
            albedo_texture: None,
            occlusion_metallic_roughness_texture: None,
            metallic_roughness_texture: None,
            occlusion_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            occlusion_strength: 1.0,
            normal_texture: None,
            normal_scale: 1.0,
            emissive: Srgba::BLACK,
            emissive_texture: None,
            index_of_refraction: 1.5,
            transmission: 0.0,
            transmission_texture: None,
            alpha_cutout: None,
            lighting_model: LightingModel::Blinn,
        }
    }
}

pub enum MaterialType {
    Transparent,
    Opaque,
}

/// Struct for easier creation of physical materials.
pub struct MaterialBuilder {
    material_type: Option<MaterialType>,
    pub name: String,
    /// Albedo base color, also called diffuse color.
    pub albedo: Srgba,
    /// Texture with albedo base colors, also called diffuse colors.
    /// The colors are assumed to be in sRGB (`RgbU8`), sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub albedo_texture: Option<Texture2D>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the occlusion, metallic and roughness parameters.
    /// The occlusion values are sampled from the red channel, metallic from the blue channel and the roughness from the green channel.
    pub occlusion_metallic_roughness_texture: Option<Texture2D>,
    pub metallic_roughness_texture: Option<Texture2D>,
    /// A scalar multiplier controlling the amount of occlusion applied from the [Self::occlusion_texture]. A value of 0.0 means no occlusion. A value of 1.0 means full occlusion.
    pub occlusion_strength: f32,
    /// An occlusion map. Higher values indicate areas that should receive full indirect lighting, and lower values indicate no indirect lighting.
    /// The occlusion values are sampled from the red channel.
    /// Can be combined with metallic and roughness into one texture, see [Self::occlusion_metallic_roughness_texture].
    pub occlusion_texture: Option<Texture2D>,
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Texture2D>,
    /// Color of light shining from an object.
    pub emissive: Srgba,
    /// Texture with the color of light shining from an object.
    /// The colors are assumed to be in sRGB (`RgbU8`), sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub emissive_texture: Option<Texture2D>,
    /// Alpha cutout value for transparency in deferred rendering pipeline.
    pub alpha_cutout: Option<f32>,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
    /// The index of refraction for this material
    pub index_of_refraction: f32,
    /// A value in the range `[0..1]` specifying how transmissive the material surface is.
    pub transmission: f32,
    /// Texture containing the transmission parameter which is multiplied with the [Self::transmission] to get the final parameter.
    pub transmission_texture: Option<Texture2D>,
}

impl MaterialBuilder {
    #[must_use]
    pub fn opaque(mut self) -> Self {
        self.material_type = Some(MaterialType::Opaque);
        self
    }

    #[must_use]
    pub fn transparent(mut self) -> Self {
        self.material_type = Some(MaterialType::Transparent);
        self
    }

    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    #[must_use]
    pub fn albedo(mut self, albedo: Srgba) -> Self {
        self.albedo = albedo;
        self
    }

    #[must_use]
    pub fn albedo_texture(mut self, albedo_texture: Texture2D) -> Self {
        self.albedo_texture = Some(albedo_texture);
        self
    }

    #[must_use]
    pub fn metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }

    #[must_use]
    pub fn roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness;
        self
    }

    #[must_use]
    pub fn occlusion_metallic_roughness_texture(
        mut self,
        occlusion_metallic_roughness_texture: Texture2D,
    ) -> Self {
        self.occlusion_metallic_roughness_texture = Some(occlusion_metallic_roughness_texture);
        self
    }

    #[must_use]
    pub fn metallic_roughness_texture(mut self, metallic_roughness_texture: Texture2D) -> Self {
        self.metallic_roughness_texture = Some(metallic_roughness_texture);
        self
    }

    #[must_use]
    pub fn occlusion_strength(mut self, occlusion_strength: f32) -> Self {
        self.occlusion_strength = occlusion_strength;
        self
    }

    #[must_use]
    pub fn occlusion_texture(mut self, occlusion_texture: Texture2D) -> Self {
        self.occlusion_texture = Some(occlusion_texture);
        self
    }

    #[must_use]
    pub fn normal_scale(mut self, normal_scale: f32) -> Self {
        self.normal_scale = normal_scale;
        self
    }

    #[must_use]
    pub fn normal_texture(mut self, normal_texture: Texture2D) -> Self {
        self.normal_texture = Some(normal_texture);
        self
    }

    #[must_use]
    pub fn emissive(mut self, emissive: Srgba) -> Self {
        self.emissive = emissive;
        self
    }

    #[must_use]
    pub fn emissive_texture(mut self, emissive_texture: Texture2D) -> Self {
        self.emissive_texture = Some(emissive_texture);
        self
    }

    #[must_use]
    pub fn alpha_cutout(mut self, alpha_cutout: f32) -> Self {
        self.alpha_cutout = Some(alpha_cutout);
        self
    }

    #[must_use]
    pub fn lighting_model(mut self, lighting_model: LightingModel) -> Self {
        self.lighting_model = lighting_model;
        self
    }

    #[must_use]
    pub fn index_of_refraction(mut self, index_of_refraction: f32) -> Self {
        self.index_of_refraction = index_of_refraction;
        self
    }

    #[must_use]
    pub fn transmission(mut self, transmission: f32) -> Self {
        self.transmission = transmission;
        self
    }

    #[must_use]
    pub fn transmission_texture(mut self, transmission_texture: Texture2D) -> Self {
        self.transmission_texture = Some(transmission_texture);
        self
    }

    pub fn build(self, ctx: &Context) -> PhysicalMaterial {
        let pbr_material = PbrMaterial {
            name: self.name,
            albedo: self.albedo,
            albedo_texture: self.albedo_texture,
            metallic: self.metallic,
            roughness: self.roughness,
            occlusion_metallic_roughness_texture: self.occlusion_metallic_roughness_texture,
            metallic_roughness_texture: self.metallic_roughness_texture,
            occlusion_strength: self.occlusion_strength,
            occlusion_texture: self.occlusion_texture,
            normal_scale: self.normal_scale,
            normal_texture: self.normal_texture,
            emissive: self.emissive,
            emissive_texture: self.emissive_texture,
            alpha_cutout: self.alpha_cutout,
            lighting_model: self.lighting_model,
            index_of_refraction: self.index_of_refraction,
            transmission: self.transmission,
            transmission_texture: self.transmission_texture,
        };

        if let Some(material_type) = self.material_type {
            match material_type {
                MaterialType::Opaque => PhysicalMaterial::new_opaque(ctx, &pbr_material),
                MaterialType::Transparent => PhysicalMaterial::new_transparent(ctx, &pbr_material),
            }
        } else {
            // detects the type based on the pbr material
            PhysicalMaterial::new(ctx, &pbr_material)
        }
    }
}
