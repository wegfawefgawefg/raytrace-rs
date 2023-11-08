use glam::Vec3;
use rand::Rng;

use crate::structures::{Light, Material, Sphere}; // Rng trait provides methods for random number generation

pub struct Scene {
    pub width: f32,
    pub height: f32,
    pub cam: Vec3,
    pub lights: Vec<Light>,
    pub shapes: Vec<Sphere>,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Scene {
        let mut rng = rand::thread_rng(); // RNG for random number generation

        let mut lights = Vec::new();
        let mut shapes = Vec::new();

        // spheres
        for _ in 0..30 {
            let material = Material::new(
                Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 255.0,
                0.05,
                0.25,
                0.1,
                1.0,
            );

            let sphere = Sphere::new(
                Vec3::new(
                    rng.gen::<f32>() * width,
                    rng.gen::<f32>() * height,
                    width / 2.0 + rng.gen::<f32>() * width,
                ),
                rng.gen::<f32>() * width / 7.0,
                material,
            );
            shapes.push(sphere);
        }

        // lights
        for _ in 0..20 {
            let light = Light::new(
                Vec3::new(
                    width / 2.0 + (rng.gen::<f32>() - 0.5) * 2.0 * width * 2.0,
                    height / 2.0 + (rng.gen::<f32>() - 0.5) * 2.0 * height * 2.0,
                    width / 2.0 + rng.gen::<f32>() * width,
                ),
                Vec3::new(rng.gen(), rng.gen(), rng.gen()),
            );
            lights.push(light);
        }

        Scene {
            width,
            height,
            cam: Vec3::new(width / 2.0, height / 2.0, -width / 2.0),
            lights,
            shapes,
        }
    }
}

// Assume Sphere and Material structs and their implementations are already defined above.
