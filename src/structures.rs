extern crate glam;

use glam::{Vec3, Vec3A}; // Vec3A could provide better performance due to alignment

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin,
            dir: dir.normalize(),
        }
    }
}

pub struct Material {
    pub color: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub reflective: f32,
}

impl Material {
    pub fn new(
        color: Vec3,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        reflective: f32,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            reflective,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Vec3::new(0.0, 0.0, 255.0), 0.05, 0.25, 0.1, 1.0)
    }
}

#[derive(Clone)]
pub struct Light {
    pub pos: Vec3,
    pub color: Vec3,
}

impl Light {
    pub fn new(pos: Vec3, color: Vec3) -> Light {
        Light { pos, color }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<f32> {
        let to_sphere = self.center - ray.origin;
        let t = to_sphere.dot(ray.dir);
        if t < 0.0 {
            return None;
        }
        let perp_point = ray.origin + ray.dir * t;
        let shortest_line = perp_point - self.center;
        let y = shortest_line.length();
        if y <= self.radius {
            let x = (self.radius.powi(2) - y.powi(2)).sqrt();
            let dist = t - x;
            Some(dist)
        } else {
            None
        }
    }

    pub fn get_normal(&self, hit_pos: Vec3) -> Vec3 {
        (hit_pos - self.center).normalize()
    }
}
