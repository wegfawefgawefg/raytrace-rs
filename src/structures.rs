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

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
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

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
