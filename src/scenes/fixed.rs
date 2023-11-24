use glam::Vec3;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::f32::consts::PI;

use crate::material::{BasicMaterial, CheckerMaterial};
use crate::scene::Scene;
use crate::shapes::Quad;
use crate::shapes::{Plane, Sphere};
use crate::structures::Light;

pub fn single_centered_light(scene: &mut Scene) {
    let light = Light::new(
        Vec3::new(0.5, 0.5, 0.5) * scene.scale * 5.0,
        Vec3::new(255.0, 255.0, 255.0),
    );
    scene.lights.push(light);
}

pub fn quad_light(scene: &mut Scene) {
    let vertical_offset = 2.0 * scene.scale;
    let lateral_offset = 2.0 * scene.scale;
    scene.lights.push(Light::new(
        Vec3::new(lateral_offset, vertical_offset, lateral_offset),
        Vec3::new(255.0, 255.0, 255.0),
    ));
    scene.lights.push(Light::new(
        Vec3::new(-lateral_offset, vertical_offset, -lateral_offset),
        Vec3::new(255.0, 255.0, 255.0),
    ));
    scene.lights.push(Light::new(
        Vec3::new(lateral_offset, vertical_offset, -lateral_offset),
        Vec3::new(255.0, 255.0, 255.0),
    ));
    scene.lights.push(Light::new(
        Vec3::new(-lateral_offset, vertical_offset, lateral_offset),
        Vec3::new(255.0, 255.0, 255.0),
    ));
}

pub fn some_random_lights(scene: &mut Scene) {
    let seed = [0u8; 32]; // All zeros
    let mut rng = SmallRng::from_seed(seed);

    for _ in 0..3 {
        let light = Light::new(
            Vec3::new(
                (rng.gen::<f32>() - 0.5) * scene.scale,
                (rng.gen::<f32>() - 0.5) * scene.scale,
                rng.gen::<f32>() * scene.scale,
            ),
            Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        );
        scene.lights.push(light);
    }
}

pub fn some_random_balls(scene: &mut Scene) {
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
            0.0,
            0.0,
        );

        let sphere = Sphere::new(
            Vec3::new(
                (rng.gen::<f32>() - 0.5) * scene.scale * 2.0,
                (rng.gen::<f32>() - 0.5) * scene.scale * 2.0,
                (rng.gen::<f32>() - 0.5) * scene.scale * 2.0,
            ),
            (rng.gen::<f32>() - 0.0) * scene.scale / 8.0 + scene.scale / 16.0,
            Box::new(material),
        );
        scene.shapes.push(Box::new(sphere));
    }
}

pub fn test_balls(scene: &mut Scene) {
    // center
    scene.shapes.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        // Vec3::new(0.0, -scene.scale / 20.0, 0.0),
        scene.scale / 4.0,
        Box::new(BasicMaterial::new(
            Vec3::new(255.0, 255.0, 255.0),
            0.3,
            0.0,
            0.0,
            0.0,
            1.0,
            0.1,
        )),
    )));

    // behind left
    scene.shapes.push(Box::new(Sphere::new(
        Vec3::new(-scene.scale / 4.0, 0.0, scene.scale / 4.0),
        scene.scale / 4.0,
        Box::new(BasicMaterial::new(
            Vec3::new(255.0, 100.0, 100.0),
            0.25,
            0.1,
            0.1,
            0.0,
            0.0,
            0.0,
        )),
    )));

    // behind right
    scene.shapes.push(Box::new(Sphere::new(
        Vec3::new(scene.scale / 4.0, 0.0, scene.scale / 2.0),
        scene.scale / 4.0,
        Box::new(BasicMaterial::new(
            Vec3::new(100.0, 255.0, 100.0),
            0.0,
            0.25,
            0.1,
            0.0,
            0.0,
            0.0,
        )),
    )));

    // up and too the right
    scene.shapes.push(Box::new(Sphere::new(
        Vec3::new(scene.scale / 2.0, scene.scale / 3.0, scene.scale / 4.0),
        scene.scale / 6.0,
        Box::new(BasicMaterial::new(
            Vec3::new(255.0, 255.0, 255.0),
            0.0,
            0.25,
            0.1,
            0.3,
            0.0,
            0.0,
        )),
    )));
}

pub fn infinite_checkered_floor(scene: &mut Scene) {
    // a plane
    let basic_material =
        BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0, 0.0, 0.0);
    let material = CheckerMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        Vec3::new(0.0, 0.0, 0.0),
        scene.scale,
        basic_material,
    );
    let plane = Plane::new(
        Vec3::new(0.0, -scene.scale, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Box::new(material),
    );
    scene.shapes.push(Box::new(plane));
}

pub fn checkered_floor(scene: &mut Scene) {
    // a plane
    let basic_material =
        BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0, 0.0, 0.0);
    let material = CheckerMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        Vec3::new(0.0, 0.0, 0.0),
        scene.scale / 2.0,
        basic_material,
    );

    let size = scene.scale * 5.0;

    let plane = Quad::new(
        Vec3::new(-size / 2.0, -scene.scale * 0.5, -size / 2.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(size, 0.0, 0.0),
        Vec3::new(0.0, 0.0, size),
        Box::new(material),
    );
    scene.shapes.push(Box::new(plane));
}

pub fn matte_floor(scene: &mut Scene) {
    // a plane
    let material = BasicMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        0.05,
        0.1,
        0.01,
        0.05,
        0.0,
        0.0,
    );
    let plane = Plane::new(
        Vec3::new(0.0, -scene.scale, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Box::new(material),
    );
    scene.shapes.push(Box::new(plane));
}

pub fn scene_4(scene: &mut Scene) {
    let scene_center = Vec3::ZERO;

    // one light at 000
    let light = Light::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0));
    scene.lights.push(light);

    // lights
    let offset = scene.scale / 2.0;
    for i in 0..6 {
        let t = PI / 3.0 * i as f32;
        let offset_x_mod = t.cos() * offset;
        let offset_y_mod = t.sin() * offset;
        let p = scene_center + Vec3::new(offset_x_mod, 0.0, offset_y_mod);

        // make a ring of lights
        let light = Light::new(p, Vec3::new(1.0, 1.0, 1.0));
        scene.lights.push(light);

        // make a ring of spheres
        let basic_material =
            BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0, 0.0, 0.0);
        let material = CheckerMaterial::new(
            Vec3::new(255.0, 255.0, 255.0),
            Vec3::new(0.0, 0.0, 0.0),
            scene.scale / 8.0,
            basic_material,
        );
        let sphere = Sphere::new(p, scene.scale / 16.0, Box::new(material));
        scene.shapes.push(Box::new(sphere));
    }

    // single centered sphere
    let basic_material =
        BasicMaterial::new(Vec3::new(255.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0, 0.0, 0.0);
    let material = CheckerMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        Vec3::new(0.0, 0.0, 0.0),
        scene.scale / 8.0,
        basic_material,
    );
    let sphere = Sphere::new(Vec3::ZERO, scene.scale / 4.0, Box::new(material));
    scene.shapes.push(Box::new(sphere));
}

pub fn centered_ball(scene: &mut Scene) {
    let material = BasicMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        0.0,
        0.25,
        0.1,
        1.0,
        0.0,
        0.0,
    );
    let sphere = Sphere::new(Vec3::ZERO, scene.scale / 2.0, Box::new(material));
    scene.shapes.push(Box::new(sphere));
}

pub fn set_cam(scene: &mut Scene) {
    let center = Vec3::ZERO;

    // scene.cam.pos.x = 0.0;
    // scene.cam.pos.z = center.z;
    // scene.cam.look_at(center);
}

pub fn basic_quad(scene: &mut Scene) {
    scene.shapes.push(Box::new(Quad::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 0.1, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Box::new(BasicMaterial::new(
            Vec3::new(255.0, 255.0, 255.0),
            0.5,
            0.8,
            0.05,
            0.2,
            0.0,
            0.0,
        )),
    )));
}

pub fn light_box(scene: &mut Scene) {
    let material = BasicMaterial::new(
        Vec3::new(255.0, 255.0, 255.0),
        0.00,
        0.01,
        0.1,
        1.0,
        0.0,
        0.0,
    );

    let scale = 2.0 * scene.scale;
    let width = scale * 1.0;
    let height = scale * 1.0;

    // back wall
    scene.shapes.push(Box::new(Quad::new(
        Vec3::new(-width / 2.0, -width / 2.0, width / 2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, height, 0.0),
        Box::new(material.clone()),
    )));

    // Left Wall
    scene.shapes.push(Box::new(Quad::new(
        Vec3::new(-width / 2.0, -height * 0.5, width / 2.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -width),
        Vec3::new(0.0, height, 0.0),
        Box::new(material.clone()),
    )));

    // Right Wall
    scene.shapes.push(Box::new(Quad::new(
        Vec3::new(width / 2.0, -scale * 0.5, width / 2.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -width),
        Vec3::new(0.0, height, 0.0),
        Box::new(material.clone()),
    )));

    // Top Wall (Ceiling)
    scene.shapes.push(Box::new(Quad::new(
        Vec3::new(-width / 2.0, height * 0.5, width / 2.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -width),
        Box::new(material.clone()),
    )));

    // Bottom Wall (Floor)
    scene.shapes.push(Box::new(Quad::new(
        Vec3::new(-width / 2.0, -height * 0.5, width / 2.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(width, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -width),
        Box::new(material.clone()),
    )));
}

pub fn raised_cam(scene: &mut Scene) {
    scene.cam.pos.y = scene.scale / 6.0;
}
