use std::f32::consts::PI;

use indicatif::ProgressBar;

use glam::{IVec2, Vec3};
use image_writing::write_as_png;
use rendering::{render_scene, render_scene_no_pb};
use scene::Scene;
use structures::{Light, Sphere};

pub mod image_writing;
pub mod rendering;
pub mod scene;
pub mod structures;

fn main() {
    // let dims = IVec2::new(200, 200);
    // let pixels = vec![vec![IVec3 { x: 255, y: 0, z: 0 }; dims.x as usize]; dims.y as usize];
    let resolutions = [
        IVec2 { x: 20, y: 20 },
        IVec2 { x: 200, y: 200 },
        IVec2 { x: 500, y: 500 },
        IVec2 { x: 1920, y: 1080 },
        IVec2 {
            x: 1920 * 2,
            y: 2160,
        },
        IVec2 { x: 4096, y: 2160 },
        IVec2 { x: 7680, y: 4320 },
    ];
    let resolution = resolutions[2];
    // basic_balls(resolution);
    make_animation(resolution, 60);
}

pub fn basic_balls(resolution: IVec2) {
    let scene = Scene::new(resolution.x as f32, resolution.y as f32);
    let pixels = render_scene(&scene, 5);

    write_as_png("output", &pixels, resolution.x as u32, resolution.y as u32)
        .expect("Failed to write PNG file");
}

pub fn make_animation(resolution: IVec2, num_frames: u32) {
    // make folder for animation
    let path = std::path::Path::new("animation");
    if path.exists() {
        std::fs::remove_dir_all(path).expect("Failed to remove animation folder");
    }
    std::fs::create_dir_all("animation").expect("Failed to create animation folder");

    // procedurally generate frames
    let width = resolution.x as f32;
    let height = resolution.y as f32;
    let start_time = 0.0;
    let end_time = PI * 2.0;
    let interval = (end_time - start_time) / num_frames as f32;
    let pb = ProgressBar::new(num_frames as u64);
    for i in 0..num_frames {
        let t = start_time + i as f32 * interval;

        // make a scene
        let mut scene = Scene {
            width,
            height,
            cam: Scene::default_cam(width, height),
            lights: vec![],
            shapes: vec![],
        };

        // a single centered light
        let light = Light::new(
            // Vec3::new(
            //     width / 2.0 + 0.5 * 2.0 * width * 2.0,
            //     height / 2.0 + 0.5 * 2.0 * height * 2.0,
            //     width / 2.0 + 0.5 * width,
            // ),
            Vec3::new(width / 4.0, height / 4.0, 0.0),
            Vec3::new(255.0, 255.0, 255.0),
        );
        scene.lights.push(light);

        // sphere material
        let material = structures::Material::new(
            Vec3::new(150.0, 150.0, 200.0),
            0.1, //0.05
            0.5,
            0.8,
            1.0,
        );

        // lets make a sphere go around in a circle around the center of the screen
        let offset = width / 4.0;
        let scene_center = Vec3::new(width / 2.0, height / 2.0, 0.0);

        let offset_x_mod = t.cos() * offset;
        let offset_y_mod = t.sin() * offset;
        let p = scene_center + Vec3::new(offset_x_mod, offset_y_mod, 0.0);
        let radius = 50.0;
        let sphere = Sphere {
            center: p,
            radius,
            material,
        };
        scene.shapes.push(sphere);

        // render the scene
        let pixels = render_scene_no_pb(&scene, 3);

        // save rendered  frame
        let path = format!("animation/{}", i);
        write_as_png(&path, &pixels, resolution.x as u32, resolution.y as u32)
            .expect("Failed to write PNG file");

        pb.inc(1);
    }
    pb.finish_with_message("Animation complete");

    // run make_vid.sh
    let output = std::process::Command::new("sh")
        .arg("make_vid.sh")
        .output()
        .expect("Failed to run make_vid.sh");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
