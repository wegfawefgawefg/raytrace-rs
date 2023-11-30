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
pub mod utils;

fn main() {
    // let dims = IVec2::new(200, 200);
    // let pixels = vec![vec![IVec3 { x: 255, y: 0, z: 0 }; dims.x as usize]; dims.y as usize];
    let resolutions = [
        IVec2 { x: 20, y: 20 },
        IVec2 { x: 200, y: 200 },
        IVec2 { x: 300, y: 200 },   // 2
        IVec2 { x: 500, y: 500 },   // 3
        IVec2 { x: 1000, y: 1000 }, // 4
        IVec2 { x: 1920, y: 1080 }, // 5
        IVec2 {
            x: 1920 * 2,
            y: 2160,
        }, // 6
        IVec2 { x: 3840, y: 2160 }, // 7
        IVec2 { x: 7680, y: 4320 },
        IVec2 { x: 1080, y: 1920 }, // 9 vertical 1080 monitor
    ];

    let mut scene_builders = Vec::<SceneBuilder>::new();
    let mut procedural_scene_builders = Vec::<ProceduralSceneBuilder>::new();

    scene_builders.push(scenes::fixed::quad_light);

    // scene_builders.push(scenes::fixed::single_centered_light);
    // scene_builders.push(scenes::fixed::some_random_lights);
    // scene_builders.push(scenes::fixed::basic_quad);

    // scene_builders.push(scenes::fixed::light_box);
    // scene_builders.push(scenes::fixed::centered_ball);
    scene_builders.push(scenes::fixed::centered_ball_with_normals);

    // scene_builders.push(scenes::fixed::light_ball);
    scene_builders.push(scenes::fixed::test_balls);
    scene_builders.push(scenes::fixed::checkered_floor);
    // scene_builders.push(scenes::fixed::textured_floor);
    // scene_builders.push(scenes::fixed::matte_floor);

    scene_builders.push(scenes::fixed::sky_sphere);
    // scene_builders.push(scenes::fixed::infinite_checkered_floor);
    // scene_builders.push(scenes::fixed::raised_cam);
    // scene_builders.push(scenes::fixed::shifted_cam);

    // scene_builders.push(scenes::fixed::set_cam);
    // scene_builders.push(scenes::fixed::some_random_balls);
    // scene_builders.push(scenes::fixed::scene_4);

    ////////    STANDALONE ANIMATIONS    ////////
    // procedural_scene_builders.push(scenes::animated::interweaved_xbox_spinny);

    //////////////////////// CAMERA ZONE ////////////////////////
    procedural_scene_builders.push(scenes::animated::orbit_camera);
    // procedural_scene_builders.push(scenes::animated::pidgeon_camera);

    let resolution = resolutions[5];
    println!("Resolution: {:?}", resolution);
    let time = std::time::Instant::now();

    let samps = 20;
    let rng_seed = [0u8; 32];

    // generate_image(
    //     resolution,
    //     samps,
    //     rng_seed,
    //     scene_builders,
    //     procedural_scene_builders,
    // );

    generate::generate_animation(
        resolution,
        240,
        samps,
        rng_seed,
        scene_builders,
        procedural_scene_builders,
    );

    println!("Time elapsed: {:?}", time.elapsed());
}
