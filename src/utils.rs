use glam::{Vec2, Vec3};
use rand::{
    rngs::{SmallRng, ThreadRng},
    Rng,
};

pub const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random_vector_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_vector_in_hemisphere(normal: Vec3, rng: &mut SmallRng) -> Vec3 {
    let in_unit_sphere = random_vector_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_vector_in_unit_disk(rng: &mut SmallRng) -> Vec2 {
    loop {
        let p = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn perpendicular_to(v: Vec3) -> Vec3 {
    let up = Vec3::new(0.0, 1.0, 0.0);
    if v.cross(up).length() > 1e-6 {
        v.cross(up).normalize()
    } else {
        v.cross(Vec3::new(0.0, 0.0, 1.0)).normalize()
    }
}
