use glam::IVec2;

use crate::generate::{generate_image, ProceduralSceneBuilder, SceneBuilder};

pub mod generate;
pub mod image_writing;
pub mod material;
pub mod rendering;
pub mod scene;
pub mod scenes;
pub mod shapes;
pub mod structures;

fn main() {
    // let dims = IVec2::new(200, 200);
    // let pixels = vec![vec![IVec3 { x: 255, y: 0, z: 0 }; dims.x as usize]; dims.y as usize];
    let resolutions = [
        IVec2 { x: 20, y: 20 },
        IVec2 { x: 200, y: 200 },
        IVec2 { x: 300, y: 200 },
        IVec2 { x: 500, y: 500 },
        IVec2 { x: 1000, y: 1000 },
        IVec2 { x: 1920, y: 1080 },
        IVec2 {
            x: 1920 * 2,
            y: 2160,
        },
        IVec2 { x: 3840, y: 2160 },
        IVec2 { x: 7680, y: 4320 },
    ];
    let resolution = resolutions[7];
    println!("Resolution: {:?}", resolution);
    let time = std::time::Instant::now();

    let mut scene_builders = Vec::<SceneBuilder>::new();
    let mut procedural_scene_builders = Vec::<ProceduralSceneBuilder>::new();

    scene_builders.push(scenes::fixed::single_centered_light);
    scene_builders.push(scenes::fixed::some_random_lights);
    scene_builders.push(scenes::fixed::checkered_floor);
    // scene_builders.push(scenes::fixed::centered_ball);

    // scene_builders.push(scenes::fixed::set_cam);
    scene_builders.push(scenes::fixed::some_random_balls);
    // scene_builders.push(scenes::fixed::scene_4);

    // procedural_scene_builders.push(scenes::animated::interweaved_xbox_spinny);
    procedural_scene_builders.push(scenes::animated::up_down_camera);

    // generate_image(resolution, scene_builders, procedural_scene_builders);
    generate::generate_animation(resolution, 120, scene_builders, procedural_scene_builders);

    println!("Time elapsed: {:?}", time.elapsed());
}
