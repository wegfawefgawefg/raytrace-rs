use image::{DynamicImage, GenericImageView};
use std::sync::Arc;

use glam::{Vec2, Vec3};

pub trait Material: Sync {
    fn color_at(&self, uv: &Vec2) -> Vec3;
    fn ambient_at(&self, uv: &Vec2) -> f32;
    fn diffuse_at(&self, uv: &Vec2) -> f32;
    fn specular_at(&self, uv: &Vec2) -> f32;
    fn reflection_at(&self, uv: &Vec2) -> f32;
    fn refraction_at(&self, uv: &Vec2) -> f32;
    fn refractive_index_at(&self, uv: &Vec2) -> f32;
}

#[derive(Clone)]
pub struct BasicMaterial {
    pub color: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub reflection: f32,
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
        refraction: f32,
        refractive_index: f32,
    ) -> BasicMaterial {
        BasicMaterial {
            color,
            ambient,
            diffuse,
            specular,
            reflection,
            refraction,
            refractive_index,
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

    fn refraction_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refraction
    }

    fn refractive_index_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refractive_index
    }
}

#[derive(Clone)]
pub struct TexturedMaterial {
    texture: Arc<DynamicImage>,
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
        let texture = image::open(texture_path).expect("Failed to load texture");

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
        // Scale the UV coordinates
        let scaled_uv = Vec2::new(uv.x / self.scale.x, uv.y / self.scale.y);

        // Apply wrapping by using modulo operation
        // Check if UV is out of bounds and wrap is false
        if !self.wrap
            && (scaled_uv.x < 0.0 || scaled_uv.x > 1.0 || scaled_uv.y < 0.0 || scaled_uv.y > 1.0)
        {
            return self.basic_material.color_at(uv);
        }

        let (width, height) = self.texture.dimensions();
        let x = ((scaled_uv.x * width as f32).rem_euclid(width as f32)) as u32;
        let y = ((scaled_uv.y * height as f32).rem_euclid(height as f32)) as u32;

        let pixel = self.texture.get_pixel(x, y);
        // reconsider the value scales here
        Vec3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)
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

    fn refraction_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refraction
    }

    fn refractive_index_at(&self, _uv: &Vec2) -> f32 {
        self.basic_material.refractive_index
    }
}
