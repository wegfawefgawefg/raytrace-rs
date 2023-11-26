use indicatif::ProgressBar;

use crate::image_writing::write_as_png;
use crate::rendering::render_scene;
use crate::scene::Scene;
use glam::IVec2;

pub type SceneBuilder = fn(&mut Scene);
pub type ProceduralSceneBuilder = fn(&mut Scene, u32, u32);

pub fn generate_image(
    resolution: IVec2,
    scene_builders: Vec<SceneBuilder>,
    procedural_scene_builders: Vec<ProceduralSceneBuilder>,
) {
    let aspect_ratio = resolution.y as f32 / resolution.x as f32;
    let mut scene = Scene::new(1.0, aspect_ratio);

    for pre_scene_builder in scene_builders {
        pre_scene_builder(&mut scene);
    }

    for psb in &procedural_scene_builders {
        psb(&mut scene, 1, 0);
    }

    let pixels = crate::rendering::render_scene(&scene, resolution, 10, true);

    write_as_png("output", &pixels, resolution.x as u32, resolution.y as u32)
        .expect("Failed to write PNG file");
}

pub fn generate_animation(
    resolution: IVec2,
    num_frames: u32,
    pre_scene_builders: Vec<SceneBuilder>,
    procedural_scene_builders: Vec<ProceduralSceneBuilder>,
) {
    // clear/make folder to store frames
    let path = std::path::Path::new("animation");
    if path.exists() {
        std::fs::remove_dir_all(path).expect("Failed to remove animation folder");
    }
    std::fs::create_dir_all("animation").expect("Failed to create animation folder");

    let pb = ProgressBar::new(num_frames as u64);
    for frame in 0..num_frames {
        let aspect_ratio = resolution.y as f32 / resolution.x as f32;
        let mut scene = Scene::new(1.0, aspect_ratio);

        for pre_scene_builder in pre_scene_builders.clone() {
            pre_scene_builder(&mut scene);
        }

        for psb in &procedural_scene_builders {
            psb(&mut scene, num_frames, frame);
        }

        let pixels = render_scene(&scene, resolution, 6, true);

        // save rendered  frame
        let path = format!("animation/{}", frame);
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
