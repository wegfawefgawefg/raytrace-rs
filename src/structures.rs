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
