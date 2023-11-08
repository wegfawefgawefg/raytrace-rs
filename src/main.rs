use indicatif::ProgressBar;

use glam::{IVec2, Vec3};
use image_writing::write_as_png;
use scene::Scene;
use structures::{Ray, Sphere};

pub mod image_writing;
pub mod scene;
pub mod structures;

// dummy raytrace function

#[allow(clippy::needless_range_loop)]
fn render_scene(scene: &Scene, max_bounces: u32) -> Vec<Vec<Vec3>> {
    let mut pixels = vec![vec![Vec3::ZERO; scene.width as usize]; scene.height as usize];

    let pb = ProgressBar::new(scene.height as u64);
    for y in 0..(scene.height as usize) {
        for x in 0..(scene.width as usize) {
            let target = Vec3::new(x as f32, y as f32, 0.0);
            let ray = Ray::new(scene.cam, target - scene.cam);
            pixels[y][x] = raytrace(&ray, scene, max_bounces, 0);
        }
        pb.inc(1);
    }
    pb.finish_with_message("Rendering complete");

    pixels
}

fn raytrace(ray: &Ray, scene: &Scene, max_bounces: u32, depth: u32) -> Vec3 {
    if depth == max_bounces {
        return Vec3::ZERO;
    }

    let mut shape_hit: Option<&Sphere> = None;
    let mut min_dist = f32::INFINITY;
    for shape in &scene.shapes {
        if let Some(dist) = shape.intersects(ray) {
            if dist < min_dist {
                shape_hit = Some(shape);
                min_dist = dist;
            }
        }
    }

    match shape_hit {
        None => Vec3::ZERO,
        Some(shape) => {
            let hit_pos = ray.origin + ray.dir * min_dist;
            let hit_normal = shape.get_normal(hit_pos);
            let mut color = color_at(scene, ray, shape, &hit_pos, &hit_normal);

            let bounce_dir = ray.dir - 2.0 * ray.dir.dot(hit_normal) * hit_normal;
            let bounce_ray = Ray::new(hit_pos + hit_normal * 0.001, bounce_dir);
            color += raytrace(&bounce_ray, scene, max_bounces, depth + 1);
            color
        }
    }
}

fn color_at(
    scene: &Scene,
    ray: &Ray,
    shape_hit: &Sphere,
    hit_pos: &Vec3,
    hit_normal: &Vec3,
) -> Vec3 {
    let mut color = shape_hit.material.color * shape_hit.material.ambient;

    for light in &scene.lights {
        let to_light = (light.pos - *hit_pos).normalize();
        let to_cam = (scene.cam - *hit_pos).normalize();

        // Diffuse lighting model
        color += shape_hit.material.color
            * shape_hit.material.diffuse
            * f32::max(hit_normal.dot(to_light), 0.0);

        // Specular lighting model
        let halfway = (to_light + to_cam).normalize();
        color += light.color
            * shape_hit.material.specular
            * f32::max(hit_normal.dot(halfway), 0.0).powi(30);
    }

    color
}

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
    let resolution = resolutions[4];
    basic_balls(resolution);
}

pub fn basic_balls(resolution: IVec2) {
    let scene = Scene::new(resolution.x as f32, resolution.y as f32);
    let pixels = render_scene(&scene, 5);

    write_as_png("output", &pixels, resolution.x as u32, resolution.y as u32)
        .expect("Failed to write PNG file");
}

// pub fn make_animation(resolution: IVec2, num_frames: u32, start_time: f32, end_time: f32) {
//     // make folder for animation
//     let path = std::path::Path::new("animation");
//     if path.exists() {
//         std::fs::remove_dir_all(path).expect("Failed to remove animation folder");
//     }
//     std::fs::create_dir_all("animation").expect("Failed to create animation folder");

//     // procedurally generate frames
//     let time = start_time;
//     let interval = (end_time - start_time) / num_frames as f32;
//     for _ in 0..num_frames {
//         // make a scene

//         // render the scene

//         // save rendered  frame
//         let path = format!("animation/{}.png", time);
//         write_as_png(&path, &pixels, resolution.x as u32, resolution.y as u32)
//             .expect("Failed to write PNG file");
//     }
// }
