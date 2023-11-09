use glam::Vec3;

use crate::{shapes::Shape, structures::Light}; // Rng trait provides methods for random number generation

pub struct Scene {
    pub width: f32,
    pub height: f32,
    pub cam: Vec3,
    pub lights: Vec<Light>,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Scene {
        Scene {
            width,
            height,
            cam: Scene::default_cam(width, height),
            lights: vec![],
            shapes: vec![],
        }
    }

    pub fn default_cam(width: f32, height: f32) -> Vec3 {
        Vec3::new(width / 2.0, height / 2.0, -width / 2.0)
    }
}
