use bvh::{aabb::AABB, Point3, Vector3};
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
    fn aabb(&self) -> AABB;
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
    fn aabb(&self) -> AABB {
        let half_size = Vec3::new(self.radius, self.radius, self.radius);
        let min = self.center - half_size;
        let max = self.center + half_size;
        let min = Vector3::new(min.x, min.y, min.z);
        let max = Vector3::new(max.x, max.y, max.z);
        AABB::with_bounds(min, max)
    }

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

    pub fn new_from_points(
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        p4: Vec3,
        material: Box<dyn Material>,
    ) -> Quad {
        let edge1 = p2 - p1;
        let edge2 = p3 - p1;
        let normal = edge1.cross(edge2).normalize();

        Quad {
            point: p1,
            normal,
            edge1,
            edge2,
            material,
        }
    }
}

impl Shape for Quad {
    fn aabb(&self) -> AABB {
        let mut min_x = self.point.x;
        let mut max_x = self.point.x;
        let mut min_y = self.point.y;
        let mut max_y = self.point.y;
        let mut min_z = self.point.z;
        let mut max_z = self.point.z;

        // Check each vertex of the quad to find the min and max for each axis
        let vertices = [
            self.point,
            self.point + self.edge1,
            self.point + self.edge2,
            self.point + self.edge1 + self.edge2,
        ];

        for vertex in &vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
            min_z = min_z.min(vertex.z);
            max_z = max_z.max(vertex.z);
        }

        let min = Point3::new(min_x, min_y, min_z);
        let max = Point3::new(max_x, max_y, max_z);

        AABB::with_bounds(min, max)
    }

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
    fn aabb(&self) -> AABB {
        // Define the extent of the AABB in each axis.
        // These values should be large enough to encompass the area where the plane has an effect.
        let extent = 1e5f32;

        // Calculate the min and max points of the AABB.
        let min = self.point - Vec3::new(extent, extent, extent);
        let max = self.point + Vec3::new(extent, extent, extent);

        //convert
        let min = Vector3::new(min.x, min.y, min.z);
        let max = Vector3::new(max.x, max.y, max.z);

        // Return the AABB
        AABB::with_bounds(min, max)
    }

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

pub struct Tri {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Box<dyn Material>,
}

impl Tri {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Box<dyn Material>) -> Tri {
        Tri { a, b, c, material }
    }
}

impl Shape for Tri {
    fn aabb(&self) -> AABB {
        let mut min_x = self.a.x;
        let mut max_x = self.a.x;
        let mut min_y = self.a.y;
        let mut max_y = self.a.y;
        let mut min_z = self.a.z;
        let mut max_z = self.a.z;

        // Check each vertex of the quad to find the min and max for each axis
        let vertices = [self.a, self.b, self.c];

        for vertex in &vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
            min_z = min_z.min(vertex.z);
            max_z = max_z.max(vertex.z);
        }

        let min = Point3::new(min_x, min_y, min_z);
        let max = Point3::new(max_x, max_y, max_z);

        AABB::with_bounds(min, max)
    }

    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.dir.cross(edge2);
        let a = edge1.dot(h);

        if a.abs() < 1e-6 {
            return None; // Ray is parallel to the triangle
        }

        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(h);

        if !(0.0..=1.0).contains(&u) {
            return None; // Intersection is outside the triangle
        }

        let q = s.cross(edge1);
        let v = f * ray.dir.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None; // Intersection is outside the triangle
        }

        let t = f * edge2.dot(q);

        if t < ray_tmin || t > ray_tmax {
            return None; // Intersection is outside the valid range
        }

        let p = ray.at(t);
        let normal = edge1.cross(edge2).normalize();

        let mut hit_record = HitRecord::new();
        hit_record.t = t;
        hit_record.p = p;
        hit_record.set_face_normal(ray, normal);

        Some(hit_record)
    }
    fn get_normal(&self, _hit_pos: Vec3) -> Vec3 {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        edge1.cross(edge2).normalize()
    }
    fn material(&self) -> &Box<dyn Material> {
        &self.material
    }

    // UNTESTED
    fn get_hit_uv(&self, hit_pos: Vec3) -> Vec2 {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let hit_vec = hit_pos - self.a;

        // Calculate barycentric coordinates
        let dot00 = edge1.dot(edge1);
        let dot01 = edge1.dot(edge2);
        let dot11 = edge2.dot(edge2);
        let dot0h = edge1.dot(hit_vec);
        let dot1h = edge2.dot(hit_vec);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot0h - dot01 * dot1h) * inv_denom;
        let v = (dot00 * dot1h - dot01 * dot0h) * inv_denom;

        Vec2::new(u, v)
    }
}
