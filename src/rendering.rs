use glam::{IVec2, Vec3};
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::{
    scene::{Cam, Scene},
    shapes::Shape,
    structures::{HitRecord, Ray},
};

#[allow(clippy::needless_range_loop)]
pub fn render_scene(
    scene: &Scene,
    resolution: IVec2,
    max_bounces: u32,
    multithreaded: bool,
) -> Vec<Vec<Vec3>> {
    let mut pixels = vec![vec![Vec3::ZERO; resolution.x as usize]; resolution.y as usize];

    let cam = &scene.cam;

    let right = scene.cam.right;
    let left = -right;
    let up = scene.cam.up;
    let down = -up;

    let viewport_dims = scene.cam.viewport_dims;

    let viewport_center = cam.pos + cam.dir * cam.viewport_dist;
    let target_right_step = right * (cam.viewport_dims.x / resolution.x as f32);
    let target_down_step = down * (cam.viewport_dims.y / resolution.y as f32);

    let viewport_top_left =
        viewport_center + left * (viewport_dims.x / 2.0) + up * (viewport_dims.y / 2.0);

    if !multithreaded {
        render_scene_inner(
            scene,
            resolution,
            max_bounces,
            viewport_top_left,
            target_right_step,
            target_down_step,
        )
    } else {
        render_scene_inner_multithreaded(
            scene,
            resolution,
            max_bounces,
            viewport_top_left,
            target_right_step,
            target_down_step,
        )
    }
}

#[allow(clippy::needless_range_loop)]
pub fn render_scene_inner(
    scene: &Scene,
    resolution: IVec2,
    max_bounces: u32,

    viewport_top_left: Vec3,
    target_right_step: Vec3,
    target_down_step: Vec3,
) -> Vec<Vec<Vec3>> {
    let mut pixels = vec![vec![Vec3::ZERO; resolution.x as usize]; resolution.y as usize];

    let pb = ProgressBar::new(resolution.x as u64);
    for y in 0..(resolution.y as usize) {
        for x in 0..(resolution.x as usize) {
            let target = viewport_top_left
                + (target_right_step * (x as f32))
                + (target_down_step * (y as f32));
            let ray = Ray::new(scene.cam.pos, target - scene.cam.pos);
            pixels[y][x] = raytrace(&ray, scene, max_bounces, 0);
        }
        pb.inc(1);
    }
    pb.finish_with_message("Rendering complete");

    pixels
}

#[allow(clippy::needless_range_loop)]
pub fn render_scene_inner_multithreaded(
    scene: &Scene,
    resolution: IVec2,
    max_bounces: u32,

    viewport_top_left: Vec3,
    target_right_step: Vec3,
    target_down_step: Vec3,
) -> Vec<Vec<Vec3>> {
    let pixels: Vec<Vec<Vec3>> = (0..resolution.y as usize)
        .into_par_iter() // Parallel iterator over the rows
        .map(|y| {
            let mut row = Vec::with_capacity(resolution.x as usize);
            for x in 0..resolution.x as usize {
                let target = viewport_top_left
                    + (target_right_step * (x as f32))
                    + (target_down_step * (y as f32));
                let ray = Ray::new(scene.cam.pos, target - scene.cam.pos);
                row.push(raytrace(&ray, scene, max_bounces, 0));
            }
            row
        })
        .collect(); // Collect rows into a vector of rows
    pixels
}

#[allow(clippy::needless_range_loop)]
pub fn render_scene_inner_no_progress_bar(
    scene: &Scene,
    resolution: IVec2,
    max_bounces: u32,

    viewport_top_left: Vec3,
    target_right_step: Vec3,
    target_down_step: Vec3,
) -> Vec<Vec<Vec3>> {
    let mut pixels = vec![vec![Vec3::ZERO; resolution.x as usize]; resolution.y as usize];

    for y in 0..(resolution.y as usize) {
        for x in 0..(resolution.x as usize) {
            let target = viewport_top_left
                + (target_right_step * (x as f32))
                + (target_down_step * (y as f32));
            let ray = Ray::new(scene.cam.pos, target - scene.cam.pos);
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
    let mut hit_record = HitRecord::new();
    let mut min_dist = std::f32::INFINITY;

    for shape in &scene.shapes {
        let hit = shape.hit(ray, 0.001, std::f32::INFINITY, &mut hit_record);
        if hit && hit_record.t < min_dist {
            shape_hit = Some(shape);
            min_dist = hit_record.t;
        }
    }

    match shape_hit {
        None => Vec3::ZERO,
        Some(shape) => {
            let material = shape.material();
            let hit_pos = ray.at(hit_record.t);
            let hit_normal = hit_record.normal;

            let mut color = Vec3::ZERO;

            //////// REFLECTION ////////
            let reflectiveness = material.reflection_at(&hit_pos);
            if reflectiveness > 0.0 {
                let bounce_dir = ray.dir - 2.0 * ray.dir.dot(hit_normal) * hit_normal;
                let bounce_ray = Ray::new(hit_pos + hit_normal * 0.001, bounce_dir);
                color += raytrace(&bounce_ray, scene, max_bounces, depth + 1) * reflectiveness;
            }

            //////// REFRACTION ////////
            let refractiveness = material.refraction_at(&hit_pos);
            if refractiveness > 0.0 {
                let outside = ray.dir.dot(hit_normal) > 0.0; // Check if ray is outside the object
                let corrected_normal = if outside { hit_normal } else { -hit_normal };
                let refracted_dir = refract(
                    ray.dir,
                    corrected_normal,
                    material.refractive_index_at(&hit_pos),
                    outside,
                );

                if let Some(refracted_dir) = refracted_dir {
                    let refracted_ray = Ray::new(hit_pos + corrected_normal * 0.001, refracted_dir);
                    let refracted_color = raytrace(&refracted_ray, scene, max_bounces, depth + 1);
                    color += refracted_color * refractiveness;
                }
            }

            //////// DIRECT LIGHTING ////////
            color += color_at(scene, ray, shape, &hit_pos, &hit_normal);

            color
        }
    }
}

fn refract(incident: Vec3, normal: Vec3, refraction_index: f32, outside: bool) -> Option<Vec3> {
    let n = if outside {
        1.0 / refraction_index
    } else {
        refraction_index
    };
    let cosi = -normal.dot(incident).min(1.0).max(-1.0);
    let sin_t2 = n * n * (1.0 - cosi * cosi);

    if sin_t2 > 1.0 {
        return None; // Total internal reflection
    }
    let cos_t = (1.0 - sin_t2).sqrt();
    Some(n * incident + (n * cosi - cos_t) * normal)
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
        let to_cam = (scene.cam.pos - *hit_pos).normalize();

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
