use glam::Vec3;

use crate::{material::Material, structures::Ray};

pub trait Shape: Sync {
    fn intersects(&self, ray: &Ray) -> Option<f32>;
    fn get_normal(&self, hit_pos: Vec3) -> Vec3;
    fn material(&self) -> &Box<dyn Material>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let to_sphere = self.center - ray.origin;
        let t = to_sphere.dot(ray.dir);
        if t < 0.0 {
            return None;
        }
        let perp_point = ray.origin + ray.dir * t;
        let shortest_line = perp_point - self.center;
        let y = shortest_line.length();
        if y <= self.radius {
            let x = (self.radius.powi(2) - y.powi(2)).sqrt();
            let dist = t - x;
            Some(dist)
        } else {
            None
        }
    }

    fn get_normal(&self, hit_pos: Vec3) -> Vec3 {
        (hit_pos - self.center).normalize()
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Box<dyn Material>) -> Plane {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Shape for Plane {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() > 1e-6 {
            // Check not parallel (not zero)
            let v = self.point - ray.origin;
            let distance = v.dot(self.normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn get_normal(&self, _hit_pos: Vec3) -> Vec3 {
        // For a plane, the normal is constant, no matter where you hit it.
        self.normal
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}
