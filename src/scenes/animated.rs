use std::f32::consts::PI;

use crate::material::BasicMaterial;
use crate::scene::Scene;
use crate::shapes::Sphere;
use glam::Vec3;

pub fn pidgeon_camera(scene: &mut Scene, num_frames: u32, frame: u32) {
    let start_time = 0.0;
    let end_time = PI * 2.0;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    let center = Vec3::ZERO;

    let offset = scene.scale * 0.5;

    scene.cam.pos = Vec3::new(
        center.x + t.cos() * offset,
        center.y + t.sin() * offset + 0.1,
        // center.z - scene.scale * 1.0,
        center.z - scene.scale * 1.0 + t.sin() * offset,
    );

    scene.cam.look_at(center);
}

pub fn orbit_camera(scene: &mut Scene, num_frames: u32, frame: u32) {
    let start_time = 0.0;
    let end_time = PI * 2.0;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    let center = Vec3::ZERO;

    let orbit_offset = scene.scale * 1.0;

    scene.cam.pos = Vec3::new(
        center.x + t.cos() * orbit_offset,
        center.y,
        center.z + t.sin() * orbit_offset,
    );

    scene.cam.look_at(center);
}

pub fn interweaved_xbox_spinny(scene: &mut Scene, num_frames: u32, frame: u32) {
    let center = Vec3::ZERO;
    let cam_offset = scene.scale * 0.5;
    scene.cam.pos = Vec3::new(
        center.x + cam_offset,
        center.y - cam_offset,
        center.z - cam_offset / 2.0,
    );
    scene.cam.look_at(center);

    let radius = scene.scale / 12.0;

    let start_time = 0.0;
    let end_time = PI * 1.0;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    // lets make a sphere go around in a circle around the center of the screen
    let offset = scene.scale / 4.0;
    let scene_center = Vec3::ZERO;
    // let material = Box::new(BasicMaterial::new(
    //     Vec3::new(255.0, 0.0, 0.0),
    //     0.05,
    //     0.5,
    //     0.8,
    //     1.0,
    //     0.0,
    //     0.0,
    // ));
    let material = Box::new(BasicMaterial::new(
        Vec3::new(255.0, 0.0, 255.0),
        0.00,
        0.2,
        0.5,
        1.0,
        0.5,
        0.85,
    ));
    for k in 0..6 {
        let tt = t - (PI / 3.0 * k as f32);
        let offset_x_mod = tt.cos() * offset;
        let offset_y_mod = tt.sin() * offset;
        let p = scene_center + Vec3::new(offset_x_mod, 0.0, offset_y_mod);
        let sphere = Sphere {
            center: p,
            radius,
            material: material.clone(),
        };
        scene.shapes.push(Box::new(sphere));
    }

    let scene_center = Vec3::ZERO;
    let shift = Vec3::new(0.0, 0.0, scene.scale / 4.0);
    for k in 0..6 {
        let tt = t - (PI / 3.0 * k as f32);
        let offset_x_mod = tt.cos() * offset;
        let offset_y_mod = tt.sin() * offset;
        let p = scene_center + Vec3::new(0.0, offset_x_mod, offset_y_mod) + shift;
        let sphere = Sphere {
            center: p,
            radius,
            material: material.clone(),
        };
        scene.shapes.push(Box::new(sphere));
    }
}
