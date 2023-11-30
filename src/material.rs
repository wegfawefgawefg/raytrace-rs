use image::{DynamicImage, GenericImageView};
use std::sync::Arc;

use glam::{Vec2, Vec3};

pub trait Material: Sync {
    fn color_at(&self, uv: &Vec2) -> Vec3;
    fn ambient_at(&self, uv: &Vec2) -> f32;
    fn diffuse_at(&self, uv: &Vec2) -> f32;
    fn specular_at(&self, uv: &Vec2) -> f32;
    fn reflection_at(&self, uv: &Vec2) -> f32;
    fn roughness_at(&self, uv: &Vec2) -> f32;
    fn refraction_at(&self, uv: &Vec2) -> f32;
    fn refractive_index_at(&self, uv: &Vec2) -> f32;
    fn normal_at(&self, uv: &Vec2) -> Vec3 {
        Vec3::ZERO
    }
    fn normal_map_magnitude_multiplier(&self) -> f32 {
        0.0
    }
}

#[derive(Clone)]
pub struct BasicMaterial {
    pub color: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub reflection: f32,
    pub roughness: f32,
    pub refraction: f32,
    pub refractive_index: f32,
}

impl BasicMaterial {
    pub fn new(
        color: Vec3,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        reflection: f32,
        roughness: f32,
        refraction: f32,
        refractive_index: f32,
    ) -> BasicMaterial {
        BasicMaterial {
            color,
            ambient,
            diffuse,
            specular,
            reflection,
            roughness,
            refraction,
            refractive_index,
        }
    }

    pub fn builder() -> BasicMaterialBuilder {
        BasicMaterialBuilder::default()
    }
}

// builder pattern for BasicMaterial
#[derive(Default)]
pub struct BasicMaterialBuilder {
    color: Vec3,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    reflection: f32,
    roughness: f32,
    refraction: f32,
    refractive_index: f32,
}

impl BasicMaterialBuilder {
    pub fn color(mut self, color: Vec3) -> Self {
        self.color = color;
        self
    }

    pub fn ambient(mut self, value: f32) -> Self {
        self.ambient = value;
        self
    }

    pub fn diffuse(mut self, value: f32) -> Self {
        self.diffuse = value;
        self
    }

    pub fn specular(mut self, value: f32) -> Self {
        self.specular = value;
        self
    }

    pub fn reflection(mut self, value: f32) -> Self {
        self.reflection = value;
        self
    }

    pub fn roughness(mut self, value: f32) -> Self {
        self.roughness = value;
        self
    }

    pub fn refraction(mut self, value: f32) -> Self {
        self.refraction = value;
        self
    }

    pub fn refractive_index(mut self, value: f32) -> Self {
        self.refractive_index = value;
        self
    }

    pub fn build(self) -> BasicMaterial {
        BasicMaterial {
            color: self.color,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            reflection: self.reflection,
            roughness: self.roughness,
            refraction: self.refraction,
            refractive_index: self.refractive_index,
        }
    }
}

impl Material for BasicMaterial {
    fn color_at(&self, _uv: &Vec2) -> Vec3 {
        self.color
    }

    fn ambient_at(&self, _uv: &Vec2) -> f32 {
        self.ambient
    }

    fn diffuse_at(&self, _uv: &Vec2) -> f32 {
        self.diffuse
    }

    fn specular_at(&self, _uv: &Vec2) -> f32 {
        self.specular
    }

    fn reflection_at(&self, _uv: &Vec2) -> f32 {
        self.reflection
    }

    fn roughness_at(&self, _uv: &Vec2) -> f32 {
        self.roughness
    }

    fn refraction_at(&self, _uv: &Vec2) -> f32 {
        self.refraction
    }

    fn refractive_index_at(&self, _uv: &Vec2) -> f32 {
        self.refractive_index
    }
}

#[derive(Clone)]
pub struct CheckerMaterial {
    pub color1: Vec3,
    pub color2: Vec3,
    pub scale: f32,
    pub basic_material: BasicMaterial,
}

impl CheckerMaterial {
    pub fn new(
        color1: Vec3,
        color2: Vec3,
        scale: f32,
        basic_material: BasicMaterial,
    ) -> CheckerMaterial {
        CheckerMaterial {
            color1,
            color2,
            scale,
            basic_material,
        }
    }

    fn checker_at(&self, uv: &Vec2) -> Vec3 {
        let s = self.scale;
        let pattern = ((uv.x * s).floor() as i32 + (uv.y * s).floor() as i32) % 2;
        if pattern == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}

impl Material for CheckerMaterial {
    fn color_at(&self, uv: &Vec2) -> Vec3 {
        self.checker_at(uv)
    }

    fn ambient_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.ambient_at(uv)
    }

    fn diffuse_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.diffuse_at(uv)
    }

    fn specular_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.specular_at(uv)
    }

    fn reflection_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.reflection_at(uv)
    }

    fn roughness_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.roughness_at(uv)
    }

    fn refraction_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refraction
    }

    fn refractive_index_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refractive_index
    }
}

pub fn sample_texture(
    uv: &Vec2,
    texture: &Texture,
    scale: Vec2,
    wrap: bool,
    fallback_material: &BasicMaterial,
) -> Vec3 {
    // Scale the UV coordinates
    let scaled_uv = Vec2::new(uv.x / scale.x, uv.y / scale.y);

    // Apply wrapping by using modulo operation
    // Check if UV is out of bounds and wrap is false
    if !wrap && (scaled_uv.x < 0.0 || scaled_uv.x > 1.0 || scaled_uv.y < 0.0 || scaled_uv.y > 1.0) {
        return fallback_material.color_at(uv);
    }

    let (width, height) = (texture.width, texture.height);
    let x = ((scaled_uv.x * width as f32).rem_euclid(width as f32)) as u32;
    let y = ((scaled_uv.y * height as f32).rem_euclid(height as f32)) as u32;

    let pixel = texture.get_pixel(x, y);
    // reconsider the value scales here
    Vec3::new(pixel[0], pixel[1], pixel[2])
}

#[derive(Clone)]
pub struct TexturedMaterial {
    texture: Arc<Texture>,
    scale: Vec2,
    wrap: bool,
    basic_material: BasicMaterial,
}

impl TexturedMaterial {
    pub fn new(
        texture_path: &str,
        scale: Vec2,
        wrap: bool,
        basic_material: BasicMaterial,
    ) -> TexturedMaterial {
        let dimage = image::open(texture_path).expect("Failed to load texture");
        let texture = Texture::from_image(&dimage);

        TexturedMaterial {
            texture: Arc::new(texture),
            scale,
            wrap,
            basic_material,
        }
    }

    // fn color_at(&self, uv: &Vec2) -> Vec3 {
    //     let (width, height) = self.texture.dimensions();
    //     let x = (uv.x.clamp(0.0, 1.0) * width as f32) as u32;
    //     let y = (uv.y.clamp(0.0, 1.0) * height as f32) as u32;

    //     let pixel = self.texture.get_pixel(x, y);
    //     Vec3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)
    // }

    fn color_at(&self, uv: &Vec2) -> Vec3 {
        sample_texture(
            uv,
            &self.texture,
            self.scale,
            self.wrap,
            &self.basic_material,
        )
    }
}

impl Material for TexturedMaterial {
    fn color_at(&self, uv: &Vec2) -> Vec3 {
        self.color_at(uv)
    }

    // default to basic_material for other unsampled material properties
    fn ambient_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.ambient_at(uv)
    }

    fn diffuse_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.diffuse_at(uv)
    }

    fn specular_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.specular_at(uv)
    }

    fn reflection_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.reflection_at(uv)
    }

    fn roughness_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.roughness_at(uv)
    }

    fn refraction_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refraction
    }

    fn refractive_index_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refractive_index
    }
}

#[derive(Clone)]
pub struct Texture {
    width: u32,
    height: u32,
    data: Vec<Vec3>, // Store normalized normals as Vec3 (assuming Vec3 is a struct for a 3D vector)
}

impl Texture {
    pub fn new(width: u32, height: u32, data: Vec<Vec3>) -> Texture {
        Texture {
            width,
            height,
            data,
        }
    }

    pub fn from_image(image: &DynamicImage) -> Texture {
        let (width, height) = image.dimensions();
        let mut data = Vec::new();
        for (_, _, pixel) in image.to_rgb8().enumerate_pixels() {
            data.push(Vec3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32));
        }

        Texture {
            width,
            height,
            data,
        }
    }

    pub fn remap(&mut self, current_min: f32, current_max: f32, new_min: f32, new_max: f32) {
        for pixel in &mut self.data {
            pixel[0] = (pixel[0] - current_min) / (current_max - current_min) * (new_max - new_min)
                + new_min;
            pixel[1] = (pixel[1] - current_min) / (current_max - current_min) * (new_max - new_min)
                + new_min;
            pixel[2] = (pixel[2] - current_min) / (current_max - current_min) * (new_max - new_min)
                + new_min;
        }
    }

    pub fn normalize_from_255(&mut self) {
        self.remap(0.0, 255.0, 0.0, 1.0);
    }

    pub fn normalize_from_255_to_full_range(&mut self) {
        self.remap(0.0, 255.0, -1.0, 1.0)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Vec3 {
        self.data[(y * self.width + x) as usize]
    }
}

#[derive(Clone)]
pub struct TexturedMaterialWithNormal {
    texture: Arc<Texture>,
    normal_map: Arc<Texture>,
    scale: Vec2,
    wrap: bool,
    normal_map_magnitude_multiplier: f32,
    basic_material: BasicMaterial,
}

impl TexturedMaterialWithNormal {
    pub fn new(
        texture_path: &str,
        normal_map_path: &str,
        scale: Vec2,
        wrap: bool,
        normal_map_magnitude_multiplier: f32,
        basic_material: BasicMaterial,
    ) -> TexturedMaterialWithNormal {
        let dimage = image::open(texture_path).expect("Failed to load texture");
        let texture = Texture::from_image(&dimage);

        let normal_map_image = image::open(normal_map_path).expect("Failed to load normal map");

        // Construct the normal map
        let mut normal_map = Texture::from_image(&normal_map_image);
        normal_map.normalize_from_255_to_full_range();

        TexturedMaterialWithNormal {
            texture: Arc::new(texture),
            normal_map: Arc::new(normal_map),
            scale,
            wrap,
            normal_map_magnitude_multiplier,
            basic_material,
        }
    }

    // fn color_at(&self, uv: &Vec2) -> Vec3 {
    //     let (width, height) = self.texture.dimensions();
    //     let x = (uv.x.clamp(0.0, 1.0) * width as f32) as u32;
    //     let y = (uv.y.clamp(0.0, 1.0) * height as f32) as u32;

    //     let pixel = self.texture.get_pixel(x, y);
    //     Vec3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)
    // }

    fn color_at(&self, uv: &Vec2) -> Vec3 {
        sample_texture(
            uv,
            &self.texture,
            self.scale,
            self.wrap,
            &self.basic_material,
        )
    }

    fn normal_at(&self, uv: &Vec2) -> Vec3 {
        sample_texture(
            uv,
            &self.normal_map,
            self.scale,
            self.wrap,
            &self.basic_material,
        )
    }
}

impl Material for TexturedMaterialWithNormal {
    fn color_at(&self, uv: &Vec2) -> Vec3 {
        self.color_at(uv)
    }

    // default to basic_material for other unsampled material properties
    fn ambient_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.ambient_at(uv)
    }

    fn diffuse_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.diffuse_at(uv)
    }

    fn specular_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.specular_at(uv)
    }

    fn reflection_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.reflection_at(uv)
    }

    fn roughness_at(&self, uv: &Vec2) -> f32 {
        self.basic_material.roughness_at(uv)
    }

    fn refraction_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refraction
    }

    fn refractive_index_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refractive_index
    }

    fn normal_at(&self, uv: &Vec2) -> Vec3 {
        self.normal_at(uv)
    }

    fn normal_map_magnitude_multiplier(&self) -> f32 {
        self.normal_map_magnitude_multiplier
    }
}
