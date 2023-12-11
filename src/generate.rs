use indicatif::ProgressBar;

use crate::image_writing::write_as_png;
use crate::scene::{Scene, SceneBuilder};
use glam::IVec2;

pub type SceneModifier = fn(&mut Scene);
pub type ProceduralSceneModifier = fn(&mut Scene, u32, u32);

pub fn generate_image(
    resolution: IVec2,
    num_samples_per_pixel: u32,
    rng_seed: [u8; 32],
    scene_builder: &SceneBuilder,
) {
    let scene = scene_builder.generate_static();
    let optimized_scene = scene.optimize();

    const MULTITHREADED: bool = true;
    const USE_PROGRESS_BAR: bool = true;
    let pixels = crate::rendering::render_scene(
        &optimized_scene,
        resolution,
        num_samples_per_pixel,
        6,
        rng_seed,
        MULTITHREADED,
        USE_PROGRESS_BAR,
    );

    write_as_png("output", &pixels, resolution.x as u32, resolution.y as u32)
        .expect("Failed to write PNG file");
}

pub fn generate_animation(
    resolution: IVec2,
    num_frames: u32,
    num_samples_per_pixel: u32,
    rng_seed: [u8; 32],

    scene_builder: &SceneBuilder,
) {
    // clear/make folder to store frames
    let path = std::path::Path::new("animation");
    if path.exists() {
        std::fs::remove_dir_all(path).expect("Failed to remove animation folder");
    }
    std::fs::create_dir_all("animation").expect("Failed to create animation folder");

    let pb = ProgressBar::new(num_frames as u64);
    for frame in 0..num_frames {
        let scene = scene_builder.generate(num_frames, frame);
        let optimized_scene = scene.optimize();

        let pixels = crate::rendering::render_scene(
            &optimized_scene,
            resolution,
            num_samples_per_pixel,
            6,
            rng_seed,
            true,
            false,
        );
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
