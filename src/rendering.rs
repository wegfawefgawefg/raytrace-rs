use glam::Vec3;
use indicatif::ProgressBar;

use crate::{
    scene::Scene,
    structures::{Ray, Sphere},
};

#[allow(clippy::needless_range_loop)]
pub fn render_scene(scene: &Scene, max_bounces: u32) -> Vec<Vec<Vec3>> {
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

#[allow(clippy::needless_range_loop)]
pub fn render_scene_no_pb(scene: &Scene, max_bounces: u32) -> Vec<Vec<Vec3>> {
    let mut pixels = vec![vec![Vec3::ZERO; scene.width as usize]; scene.height as usize];

    for y in 0..(scene.height as usize) {
        for x in 0..(scene.width as usize) {
            let target = Vec3::new(x as f32, y as f32, 0.0);
            let ray = Ray::new(scene.cam, target - scene.cam);
            pixels[y][x] = raytrace(&ray, scene, max_bounces, 0);
        }
    }

    pixels
}

pub fn raytrace(ray: &Ray, scene: &Scene, max_bounces: u32, depth: u32) -> Vec3 {
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

pub fn color_at(
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
