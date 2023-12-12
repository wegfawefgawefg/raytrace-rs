use glam::IVec2;

use crate::{generate::generate_image, scene::SceneBuilder};

pub mod generate;
pub mod image_writing;
pub mod material;
pub mod rendering;
pub mod scene;
pub mod scenes;
pub mod shape_bvh_node;
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

    let resolution = resolutions[7];
    let aspect_ratio = resolution.y as f32 / resolution.x as f32;
    let mut scene_builder = SceneBuilder::new(1.0, aspect_ratio);

    scene_builder.add_mod(scenes::fixed::quad_light);

    // scene_builder.add_mod(scenes::fixed::single_centered_light);
    // scene_builder.add_mod(scenes::fixed::some_random_lights);
    // scene_builder.add_mod(scenes::fixed::basic_quad);

    // scene_builder.add_mod(scenes::fixed::light_box);
    // scene_builder.add_mod(scenes::fixed::centered_ball);
    // scene_builder.add_mod(scenes::fixed::centered_ball_with_normals);

    // scene_builder.add_mod(scenes::fixed::light_ball);
    // scene_builder.add_mod(scenes::fixed::test_balls);
    // scene_builder.add_mod(scenes::fixed::test_tris);
    scene_builder.add_mod(scenes::fixed::checkered_floor);
    // scene_builder.add_mod(scenes::fixed::textured_floor);
    // scene_builder.add_mod(scenes::fixed::matte_floor);

    scene_builder.add_mod(scenes::fixed::sky_sphere);
    scene_builder.add_mod(scenes::fixed::duck);

    // scene_builder.add_mod(scenes::fixed::infinite_checkered_floor);
    // scene_builder.add_mod(scenes::fixed::raised_cam);
    // scene_builder.add_mod(scenes::fixed::shifted_cam);

    // scene_builder.add_mod(scenes::fixed::set_cam);
    // scene_builder.add_mod(scenes::fixed::set_cam_raised_looking_down);
    // scene_builder.add_mod(scenes::fixed::grid_of_balls);
    // scene_builder.add_mod(scenes::fixed::some_random_balls);
    // scene_builder.add_mod(scenes::fixed::scene_4);

    ////////    STANDALONE ANIMATIONS    ////////
    // scene_builder.add_proc_mod(scenes::animated::interweaved_xbox_spinny);
    // scene_builder.add_proc_mod(scenes::animated::wave_sheet);

    //////////////////////// CAMERA ZONE ////////////////////////
    scene_builder.add_proc_mod(scenes::animated::orbit_camera);
    // scene_builder.add_proc_mod(scenes::animated::pidgeon_camera);

    let time = std::time::Instant::now();

    let samps = 1;
    println!("Resolution: {:?} @ {} samples per pixel", resolution, samps);

    let rng_seed = [0u8; 32];

    // generate_image(resolution, samps, rng_seed, &scene_builder);
    generate::generate_animation(resolution, 240, samps, rng_seed, &scene_builder);

    println!("Time elapsed: {:?}", time.elapsed());
}
