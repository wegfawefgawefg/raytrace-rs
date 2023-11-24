use glam::Vec3;

use crate::{
    material::Material,
    structures::{HitRecord, Ray},
};

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
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
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let to_sphere = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = to_sphere.dot(ray.dir);
        let c = to_sphere.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < ray_tmin || ray_tmax < root {
            root = (-half_b + sqrt_d) / a;
            if root < ray_tmin || ray_tmax < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(root);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        true
    }

    fn get_normal(&self, hit_pos: Vec3) -> Vec3 {
        (hit_pos - self.center).normalize()
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}

pub struct Quad {
    pub point: Vec3,
    pub normal: Vec3,
    pub edge1: Vec3,
    pub edge2: Vec3,
    pub material: Box<dyn Material>,
}

impl Quad {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        edge1: Vec3,
        edge2: Vec3,
        material: Box<dyn Material>,
    ) -> Quad {
        Quad {
            point,
            normal: normal.normalize(),
            edge1,
            edge2,
            material,
        }
    }
}
impl Shape for Quad {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let denominator = self.normal.dot(ray.dir);
        if denominator.abs() < 1e-6 {
            // Ray is parallel to the quad's plane
            return false;
        }

        let v = self.point - ray.origin;
        let t = v.dot(self.normal) / denominator;
        if t < 0.0 {
            // The intersection is behind the ray's origin
            return false;
        }

        let hit_point = ray.origin + ray.dir * t;

        // Check if the hit point is inside the quad bounds
        let hp_local = hit_point - self.point;
        let dot1 = hp_local.dot(self.edge1);
        let dot2 = hp_local.dot(self.edge2);
        let edge1_length_sq = self.edge1.length_squared();
        let edge2_length_sq = self.edge2.length_squared();
        if dot1 >= 0.0 && dot1 <= edge1_length_sq && dot2 >= 0.0 && dot2 <= edge2_length_sq {
            // hit
            hit_record.t = t;
            hit_record.p = hit_point;
            hit_record.set_face_normal(ray, self.normal);

            true
        } else {
            false // miss
        }
    }

    fn get_normal(&self, _hit_pos: Vec3) -> Vec3 {
        self.normal
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
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() > 1e-6 {
            // Check not parallel (not zero)
            let v = self.point - ray.origin;
            let distance = v.dot(self.normal) / denom;
            if distance >= 0.0 {
                hit_record.t = distance;
                hit_record.p = ray.at(distance);
                hit_record.set_face_normal(ray, self.normal);

                return true;
            }
        }
        false
    }

    fn get_normal(&self, _hit_pos: Vec3) -> Vec3 {
        // For a plane, the normal is constant, no matter where you hit it.
        self.normal
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}
