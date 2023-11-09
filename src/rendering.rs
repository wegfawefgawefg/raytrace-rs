use glam::Vec3;
use indicatif::ProgressBar;

use crate::{scene::Scene, shapes::Shape, structures::Ray};

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

use rayon::prelude::*;
#[allow(clippy::needless_range_loop)]
pub fn render_scene_par(scene: &Scene, max_bounces: u32) -> Vec<Vec<Vec3>> {
    let pixels: Vec<Vec<Vec3>> = (0..scene.height as usize)
        .into_par_iter() // Parallel iterator over the rows
        .map(|y| {
            let mut row = Vec::with_capacity(scene.width as usize);
            for x in 0..scene.width as usize {
                let target = Vec3::new(x as f32, y as f32, 0.0);
                let ray = Ray::new(scene.cam, target - scene.cam);
                row.push(raytrace(&ray, scene, max_bounces, 0));
            }
            row
        })
        .collect(); // Collect rows into a vector of rows
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

    let mut shape_hit: Option<&Box<dyn Shape>> = None;
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
    shape_hit: &Box<dyn Shape>,
    hit_pos: &Vec3,
    hit_normal: &Vec3,
) -> Vec3 {
    let material = shape_hit.material();
    let mut color = material.color_at(hit_pos) * material.ambient_at(hit_pos);

    for light in &scene.lights {
        let to_light = (light.pos - *hit_pos).normalize();
        let to_cam = (scene.cam - *hit_pos).normalize();

        // Diffuse lighting model
        color += material.color_at(hit_pos)
            * material.diffuse_at(hit_pos)
            * f32::max(hit_normal.dot(to_light), 0.0);

        // Specular lighting model
        let halfway = (to_light + to_cam).normalize();
        color += light.color
            * material.specular_at(hit_pos)
            * f32::max(hit_normal.dot(halfway), 0.0).powi(30);
    }

    color
}
