use glam::{Vec2, Vec3};

use crate::{
    material::Material,
    structures::{HitRecord, Ray},
    utils::perpendicular_to,
};

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
    fn get_normal(&self, hit_pos: Vec3) -> Vec3;
    fn get_hit_uv(&self, hit_pos: Vec3) -> Vec2;
    fn material(&self) -> &Box<dyn Material>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub orientation: glam::Quat,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(
        center: Vec3,
        radius: f32,
        material: Box<dyn Material>,
        orientation: glam::Quat,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            orientation,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let to_sphere = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = to_sphere.dot(ray.dir);
        let c = to_sphere.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < ray_tmin || ray_tmax < root {
            root = (-half_b + sqrt_d) / a;
            if root < ray_tmin || ray_tmax < root {
                return None;
            }
        }

        let mut hit_record = HitRecord::new();
        hit_record.t = root;
        hit_record.p = ray.at(root);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn get_normal(&self, hit_pos: Vec3) -> Vec3 {
        (hit_pos - self.center).normalize()
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }

    fn get_hit_uv(&self, hit_pos: Vec3) -> Vec2 {
        // Convert world space hit position to local space
        let local_hit_pos = self.orientation.inverse() * (hit_pos - self.center);

        // Calculate spherical coordinates
        let theta = local_hit_pos
            .y
            .atan2((local_hit_pos.x.powi(2) + local_hit_pos.z.powi(2)).sqrt());
        let phi = local_hit_pos.z.atan2(local_hit_pos.x);

        // Map spherical coordinates to UV coordinates
        // U: Azimuthal angle normalized to [0, 1]
        // V: Inclination normalized to [0, 1]
        let u = 1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        let v = (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI;

        Vec2::new(u, 1.0 - v)
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
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.dir);
        if denominator.abs() < 1e-6 {
            // Ray is parallel to the quad's plane
            return None;
        }

        let v = self.point - ray.origin;
        let t = v.dot(self.normal) / denominator;
        if t < 0.0 {
            // The intersection is behind the ray's origin
            return None;
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
            let mut hit_record = HitRecord::new();
            hit_record.t = t;
            hit_record.p = hit_point;
            hit_record.set_face_normal(ray, self.normal);

            Some(hit_record)
        } else {
            None // miss
        }
    }

    fn get_normal(&self, _hit_pos: Vec3) -> Vec3 {
        self.normal
    }

    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }

    // UNTESTED
    fn get_hit_uv(&self, hit_pos: Vec3) -> Vec2 {
        // Transform the hit position to the local space of the quad
        let local_hit_pos = hit_pos - self.point;

        // Project the local hit position onto the edges of the quad
        let u = local_hit_pos.dot(self.edge1.normalize());
        let v = local_hit_pos.dot(self.edge2.normalize());

        // Normalize the u and v coordinates based on the length of the quad's edges
        let u_normalized = u / self.edge1.length();
        let v_normalized = v / self.edge2.length();

        Vec2::new(u_normalized, v_normalized)
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
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() > 1e-6 {
            // Check not parallel (not zero)
            let v = self.point - ray.origin;
            let distance = v.dot(self.normal) / denom;
            if distance >= 0.0 {
                let mut hit_record = HitRecord::new();
                hit_record.t = distance;
                hit_record.p = ray.at(distance);
                hit_record.set_face_normal(ray, self.normal);

                return Some(hit_record);
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

    // UNTESTED
    fn get_hit_uv(&self, hit_pos: Vec3) -> Vec2 {
        // Generate two perpendicular vectors on the plane
        let u_direction = perpendicular_to(self.normal);
        let v_direction = self.normal.cross(u_direction).normalize();

        // Project hit position onto these vectors
        let u = hit_pos.dot(u_direction);
        let v = hit_pos.dot(v_direction);

        // Optionally, modulate u and v for a repeating pattern
        let u_modulated = u % 1.0;
        let v_modulated = v % 1.0;

        Vec2::new(u_modulated, v_modulated)
    }
}
