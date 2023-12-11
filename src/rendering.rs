use either::Either;
use glam::Mat3;
use glam::Vec2;
use indicatif::ProgressIterator;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use glam::{IVec2, Vec3};
use indicatif::ParallelProgressIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

use crate::scene::OptimizedScene;
use crate::utils::random_vector_in_hemisphere;
use crate::{shapes::Shape, structures::Ray, utils::random_vector_in_unit_disk};

pub const FAUX_LIGHTING_DIFFUSION: bool = true;
pub const FAUX_LIGHTING_SPECULAR: bool = true;

#[allow(clippy::needless_range_loop)]
pub fn render_scene(
    scene: &OptimizedScene,
    resolution: IVec2,
    num_samples_per_pixel: u32,
    max_bounces: u32,
    rng_seed: [u8; 32],
    multithreaded: bool,

    use_progress_bar: bool,
) -> Vec<Vec<Vec3>> {
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
            num_samples_per_pixel,
            max_bounces,
            rng_seed,
            viewport_top_left,
            target_right_step,
            target_down_step,
            use_progress_bar,
        )
    } else {
        render_scene_inner_multithreaded(
            scene,
            resolution,
            num_samples_per_pixel,
            max_bounces,
            rng_seed,
            viewport_top_left,
            target_right_step,
            target_down_step,
            use_progress_bar,
        )
    }
}

//TODO: merge render_scene_inner and render_scene_inner_multithreaded
// they only have one single difference, which is the row_iter not being parallel

#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_range_loop)]
pub fn render_scene_inner(
    scene: &OptimizedScene,
    resolution: IVec2,
    num_samples_per_pixel: u32,
    max_bounces: u32,
    rng_seed: [u8; 32],

    viewport_top_left: Vec3,
    target_right_step: Vec3,
    target_down_step: Vec3,

    use_progress_bar: bool,
) -> Vec<Vec<Vec3>> {
    let row_iter = 0..resolution.y as usize;
    let row_iter_with_maybe_progress_bar = if use_progress_bar {
        Either::Left(row_iter.progress_count(resolution.y as u64))
    } else {
        Either::Right(row_iter)
    };

    let pixels: Vec<Vec<Vec3>> = row_iter_with_maybe_progress_bar
        .map(|y| {
            let mut rng = SmallRng::from_seed(rng_seed); //rng.gen::<f32>()
            let mut row = Vec::with_capacity(resolution.x as usize);
            for x in 0..resolution.x as usize {
                let target = viewport_top_left
                    + (target_right_step * (x as f32))
                    + (target_down_step * (y as f32));
                // .progress_count(resolution.y as u64)

                let color = if num_samples_per_pixel == 1 {
                    let ray = Ray::new(scene.cam.pos, target - scene.cam.pos);
                    raytrace(&ray, scene, max_bounces, 0, &mut rng)
                } else {
                    let mut color = Vec3::ZERO;

                    for _ in 0..num_samples_per_pixel {
                        let random_offset = random_vector_in_unit_disk(&mut rng);
                        let scaled_offset = random_offset.x * target_right_step
                            + random_offset.y * target_down_step;
                        let starting_position = scene.cam.pos + scaled_offset;
                        let ray = Ray::new(starting_position, target - scene.cam.pos);
                        color += raytrace(&ray, scene, max_bounces, 0, &mut rng);
                    }
                    color /= num_samples_per_pixel as f32;
                    color
                };
                row.push(color);
            }

            row
        })
        .collect(); // Collect rows into a vector of rows
    pixels
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_range_loop)]
pub fn render_scene_inner_multithreaded(
    scene: &OptimizedScene,
    resolution: IVec2,
    num_samples_per_pixel: u32,
    max_bounces: u32,
    rng_seed: [u8; 32],

    viewport_top_left: Vec3,
    target_right_step: Vec3,
    target_down_step: Vec3,

    use_progress_bar: bool,
) -> Vec<Vec<Vec3>> {
    let row_iter = (0..resolution.y as usize).into_par_iter();
    let row_iter_with_maybe_progress_bar = if use_progress_bar {
        Either::Left(row_iter.progress_count(resolution.y as u64))
    } else {
        Either::Right(row_iter)
    };

    let pixels: Vec<Vec<Vec3>> = row_iter_with_maybe_progress_bar
        .map(|y| {
            let mut rng = SmallRng::from_seed(rng_seed); //rng.gen::<f32>()
            let mut row = Vec::with_capacity(resolution.x as usize);
            for x in 0..resolution.x as usize {
                let target = viewport_top_left
                    + (target_right_step * (x as f32))
                    + (target_down_step * (y as f32));
                // .progress_count(resolution.y as u64)

                let color = if num_samples_per_pixel == 1 {
                    let ray = Ray::new(scene.cam.pos, target - scene.cam.pos);
                    raytrace(&ray, scene, max_bounces, 0, &mut rng)
                } else {
                    let mut color = Vec3::ZERO;

                    for _ in 0..num_samples_per_pixel {
                        let random_offset = random_vector_in_unit_disk(&mut rng);
                        let scaled_offset = random_offset.x * target_right_step
                            + random_offset.y * target_down_step;
                        let starting_position = scene.cam.pos + scaled_offset;
                        let ray = Ray::new(starting_position, target - scene.cam.pos);
                        color += raytrace(&ray, scene, max_bounces, 0, &mut rng);
                    }
                    color /= num_samples_per_pixel as f32;
                    color
                };
                row.push(color);
            }

            row
        })
        .collect(); // Collect rows into a vector of rows
    pixels
}

pub fn raytrace(
    ray: &Ray,
    scene: &OptimizedScene,
    max_bounces: u32,
    depth: u32,
    rng: &mut SmallRng,
) -> Vec3 {
    if depth == max_bounces {
        return Vec3::ZERO;
    }

    let mut shape_hit: Option<&Box<dyn Shape>> = None;
    let mut closest_hit_record = None;
    let mut closest_so_far = std::f32::INFINITY;

    // old code before bvh was implemented
    // for shape in scene.get_shapes_slice() {
    //     if let Some(hit_record) = shape.hit(ray, 0.001, std::f32::INFINITY) {
    //         if hit_record.t < closest_so_far {
    //             shape_hit = Some(shape);
    //             closest_so_far = hit_record.t;
    //             closest_hit_record = Some(hit_record);
    //         }
    //     }
    // }

    // new code using bvh
    let wrapped_shapes = scene.raycast(ray);
    for wrapped_shape in wrapped_shapes {
        let shape = wrapped_shape.get_shape();
        if let Some(hit_record) = shape.hit(ray, 0.001, std::f32::INFINITY) {
            if hit_record.t < closest_so_far {
                shape_hit = Some(shape);
                closest_so_far = hit_record.t;
                closest_hit_record = Some(hit_record);
            }
        }
    }

    match shape_hit {
        None => Vec3::ZERO,
        Some(shape) => {
            let hit_record = closest_hit_record.unwrap();
            let material = shape.material();
            let mut hit_normal = hit_record.normal;
            let hit_pos = ray.at(hit_record.t);
            let uv = shape.get_hit_uv(hit_pos);

            //////// NORMAL MAPPING ////////
            if material.normal_map_magnitude_multiplier() > 0.0 {
                let mut sampled_normal = material.normal_at(&uv);
                sampled_normal = sampled_normal.normalize();

                // Calculate the TBN matrix
                let tangent = hit_normal.cross(Vec3::Y).normalize();
                let bitangent = tangent.cross(hit_normal).normalize();

                // Transform the normal from tangent space to world space
                let normal_matrix = Mat3::from_cols(tangent, bitangent, hit_normal);
                hit_normal = normal_matrix * sampled_normal;
                hit_normal = hit_normal.normalize();
            }

            let mut color = Vec3::ZERO;

            let outside = ray.dir.dot(hit_normal) < 0.0; // Check if ray is outside the object
            let corrected_normal = if outside { hit_normal } else { -hit_normal };

            //////// REFLECTION ////////
            let reflectiveness = material.reflection_at(&uv);
            if reflectiveness > 0.0 {
                // latest update is bounce_dir is randomly biased towards the hemisphere of the normal
                let mut bounce_dir =
                    ray.dir - 2.0 * ray.dir.dot(corrected_normal) * corrected_normal;

                let roughness = material.roughness_at(&uv);
                if roughness > 0.0 {
                    // let scattered_bounce_dir = random_vector_in_unit_sphere(rng) * roughness;
                    let scattered_bounce_dir = random_vector_in_hemisphere(corrected_normal, rng);
                    bounce_dir = bounce_dir.lerp(scattered_bounce_dir, roughness);
                }

                let bounce_ray = Ray::new(hit_pos + bounce_dir * 0.001, bounce_dir);
                color += raytrace(&bounce_ray, scene, max_bounces, depth + 1, rng) * reflectiveness;
            }

            //////// REFRACTION ////////
            let refractiveness = material.refraction_at(&uv);
            if refractiveness > 0.0 {
                let refracted_dir = refract(
                    ray.dir,
                    corrected_normal,
                    material.refractive_index_at(&uv),
                    outside,
                );

                if let Some(refracted_dir) = refracted_dir {
                    let refracted_ray = Ray::new(hit_pos + refracted_dir * 0.001, refracted_dir);
                    let refracted_color =
                        raytrace(&refracted_ray, scene, max_bounces, depth + 1, rng);
                    color += refracted_color * refractiveness;
                }
            }

            //////// DIRECT LIGHTING ////////
            color += color_at(scene, ray, shape, &hit_pos, &hit_normal, &uv);

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
    scene: &OptimizedScene,
    ray: &Ray,
    shape_hit: &Box<dyn Shape>,
    hit_pos: &Vec3,
    hit_normal: &Vec3,
    uv: &Vec2,
) -> Vec3 {
    let material = shape_hit.material();

    // Ambient lighting
    let mut color = material.color_at(uv) * material.ambient_at(uv);

    for light in &scene.lights {
        let to_light = (light.pos - *hit_pos).normalize();
        let to_cam = (scene.cam.pos - *hit_pos).normalize();

        // Diffuse lighting
        if FAUX_LIGHTING_DIFFUSION {
            color += material.color_at(uv)
                * material.diffuse_at(uv)
                * f32::max(hit_normal.dot(to_light), 0.0);
        }

        if FAUX_LIGHTING_SPECULAR {
            // Specular lighting
            let halfway = (to_light + to_cam).normalize();
            color += light.color
                * material.specular_at(uv)
                * f32::max(hit_normal.dot(halfway), 0.0).powi(30);
        }
    }

    color
}
