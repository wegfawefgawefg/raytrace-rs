use std::f32::consts::PI;
use std::sync::Mutex;

use crate::material::BasicMaterial;
use crate::scene::Scene;
use crate::shapes::{Quad, Sphere};
use glam::Vec3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    // let end_time = PI * 0.5;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    let center = Vec3::ZERO;

    let orbit_offset = scene.scale * 1.5;

    scene.cam.pos = Vec3::new(
        center.x + t.cos() * orbit_offset,
        center.y + scene.scale * 0.5,
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
    let material = Box::new(
        BasicMaterial::builder()
            .color(Vec3::new(255.0, 0.0, 255.0))
            .ambient(0.00)
            .diffuse(0.2)
            .specular(0.5)
            .reflection(1.0)
            .roughness(0.5)
            .refraction(0.85)
            .build(),
    );
    for k in 0..6 {
        let tt = t - (PI / 3.0 * k as f32);
        let offset_x_mod = tt.cos() * offset;
        let offset_y_mod = tt.sin() * offset;
        let p = scene_center + Vec3::new(offset_x_mod, 0.0, offset_y_mod);
        let sphere = Sphere {
            center: p,
            radius,
            material: material.clone(),
            orientation: glam::Quat::IDENTITY,
        };
        scene.add_shape(Box::new(sphere));
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
            orientation: glam::Quat::IDENTITY,
        };
        scene.add_shape(Box::new(sphere));
    }
}

pub fn wave_sheet(scene: &mut Scene, num_frames: u32, frame: u32) {
    let center = Vec3::ZERO;

    let radius = scene.scale / 12.0;

    let start_time = 0.0;
    let end_time = PI * 2.0;
    let interval = (end_time - start_time) / num_frames as f32;

    let t = start_time + frame as f32 * interval;

    // were going to make a sheet of quads, lets start with 10x10
    let mat = BasicMaterial::builder()
        .color(Vec3::new(255.0, 255.0, 255.0))
        .ambient(0.00)
        .diffuse(0.01)
        .specular(0.05)
        .reflection(0.5)
        .roughness(0.0)
        .refraction(0.85)
        .refractive_index(2.3)
        .build();

    // function which takes a point and returns a height
    let height = |x: f32, y: f32| {
        let freq = 4.0;
        (t + x * freq).sin() + (t + y * freq).cos()
    };

    let num = 100;
    let h_scale = scene.scale / 10.0;
    let quads = Mutex::new(Vec::new());

    (0..num).into_par_iter().for_each(|i| {
        (0..num).into_par_iter().for_each(|j| {
            let x = i as f32 / num as f32 - 0.5;
            let y = j as f32 / num as f32 - 0.5;

            let next_x = (i as f32 + 1.1) / num as f32 - 0.5;
            let next_y = (j as f32 + 1.1) / num as f32 - 0.5;

            let p = Vec3::new(x * scene.scale, height(x, y) * h_scale, y * scene.scale);
            let tr = Vec3::new(
                next_x * scene.scale,
                height(next_x, y) * h_scale,
                y * scene.scale,
            );
            let bl = Vec3::new(
                x * scene.scale,
                height(x, next_y) * h_scale,
                next_y * scene.scale,
            );
            let br = Vec3::new(
                next_x * scene.scale,
                height(next_x, next_y) * h_scale,
                next_y * scene.scale,
            );

            let mut quads = quads.lock().unwrap();
            quads.push((p, tr, bl, br));
        });
    });

    let quads = quads.lock().unwrap();
    for quad in &*quads {
        scene.add_shape(Box::new(Quad::new_from_points(
            quad.0,
            quad.1,
            quad.2,
            quad.3,
            Box::new(mat.clone()),
        )));
    }
}
