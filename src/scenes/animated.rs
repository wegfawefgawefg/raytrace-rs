use std::f32::consts::PI;

use crate::material::BasicMaterial;
use crate::scene::Scene;
use crate::shapes::Sphere;
use glam::Vec3;

pub fn up_down_camera(scene: &mut Scene, num_frames: u32, frame: u32) {
    let start_time = 0.0;
    let end_time = PI * 2.0;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    let center = Vec3::ZERO;

    scene.cam.pos = Vec3::new(
        center.x + t.cos() * scene.scale * 2.0,
        center.y,
        center.z + t.sin() * scene.scale * 2.0,
    );

    scene.cam.look_at(center);
}

pub fn interweaved_xbox_spinny(scene: &mut Scene, num_frames: u32, frame: u32) {
    let start_time = 0.0;
    let end_time = PI * 1.0;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    // lets make a sphere go around in a circle around the center of the screen
    let offset = scene.scale / 4.0;
    let scene_center = Vec3::ZERO;
    for k in 0..6 {
        let material = BasicMaterial::new(Vec3::new(0.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0);
        let tt = t - (PI / 3.0 * k as f32);
        let offset_x_mod = tt.cos() * offset;
        let offset_y_mod = tt.sin() * offset;
        let p = scene_center + Vec3::new(offset_x_mod, 0.0, offset_y_mod);
        let radius = scene.scale / 16.0;
        let sphere = Sphere {
            center: p,
            radius,
            material: Box::new(material),
        };
        scene.shapes.push(Box::new(sphere));
    }

    let scene_center = Vec3::ZERO;
    for k in 0..6 {
        let material = BasicMaterial::new(Vec3::new(0.0, 0.0, 0.0), 0.05, 0.5, 0.8, 1.0);
        let tt = t - (PI / 3.0 * k as f32);
        let offset_x_mod = tt.cos() * offset;
        let offset_y_mod = tt.sin() * offset;
        let p = scene_center + Vec3::new(0.0, offset_x_mod, offset_y_mod);
        let radius = scene.scale / 16.0;
        let sphere = Sphere {
            center: p,
            radius,
            material: Box::new(material),
        };
        scene.shapes.push(Box::new(sphere));
    }
}
