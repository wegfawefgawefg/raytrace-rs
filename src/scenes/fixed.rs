use glam::Vec3;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::f32::consts::PI;

use crate::material::{BasicMaterial, CheckerMaterial};
use crate::scene::Scene;
use crate::shapes::{Plane, Sphere};
use crate::structures::Light;

pub fn single_centered_light(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;

    let light = Light::new(
        Vec3::new(width / 2.0, height / 2.0, width / 2.0),
        Vec3::new(255.0, 255.0, 255.0),
    );
    scene.lights.push(light);
}

pub fn some_random_lights(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;

    let seed = [0u8; 32]; // All zeros
    let mut rng = SmallRng::from_seed(seed);

    for _ in 0..5 {
        let light = Light::new(
            Vec3::new(
                width / 2.0 + (rng.gen::<f32>() - 0.5) * 2.0 * width * 2.0,
                height / 2.0 + (rng.gen::<f32>() - 0.5) * 2.0 * height * 2.0,
                width / 2.0 + rng.gen::<f32>() * width,
            ),
            Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        );
        scene.lights.push(light);
    }
}

pub fn basic_balls(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;

    let seed = [0u8; 32]; // All zeros
    let mut rng = SmallRng::from_seed(seed);

    // spheres
    for _ in 0..30 {
        let material = BasicMaterial::new(
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
            Box::new(material),
        );
        scene.shapes.push(Box::new(sphere));
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
        scene.lights.push(light);
    }
}

pub fn checkered_floor(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;

    // a plane
    let basic_material = BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0);
    let material = CheckerMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        Vec3::new(0.0, 0.0, 0.0),
        width / 8.0,
        basic_material,
    );
    let plane = Plane::new(
        Vec3::new(width / 2.0, width / 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Box::new(material),
    );
    scene.shapes.push(Box::new(plane));
}

pub fn scene_4(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;

    let scene_center = Vec3::new(width / 2.0, height / 2.0, width / 2.0);

    // one light at 000
    let light = Light::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0));
    scene.lights.push(light);

    // lights
    let offset = width / 2.0;
    for i in 0..6 {
        let t = PI / 3.0 * i as f32;
        let offset_x_mod = t.cos() * offset;
        let offset_y_mod = t.sin() * offset;
        let p = scene_center + Vec3::new(offset_x_mod, 0.0, offset_y_mod);

        // make a ring of lights
        let light = Light::new(p, Vec3::new(1.0, 1.0, 1.0));
        scene.lights.push(light);

        // make a ring of spheres
        let basic_material = BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0);
        let material = CheckerMaterial::new(
            Vec3::new(255.0, 255.0, 255.0),
            Vec3::new(0.0, 0.0, 0.0),
            width / 8.0,
            basic_material,
        );
        let sphere = Sphere::new(p, width / 16.0, Box::new(material));
        scene.shapes.push(Box::new(sphere));
    }

    // single centered sphere
    let basic_material = BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0);
    let material = CheckerMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        Vec3::new(0.0, 0.0, 0.0),
        width / 8.0,
        basic_material,
    );
    let sphere = Sphere::new(
        Vec3::new(width / 2.0, height / 2.0, width / 2.0),
        width / 4.0,
        Box::new(material),
    );
    scene.shapes.push(Box::new(sphere));
}
