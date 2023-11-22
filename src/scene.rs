use glam::{Vec2, Vec3};

use crate::{shapes::Shape, structures::Light}; // Rng trait provides methods for random number generation

pub struct Cam {
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub viewport_dist: f32,
    pub viewport_dims: Vec2,
}

impl Cam {
    pub fn new(scale: f32, viewport_aspect_ratio: f32) -> Cam {
        // viewport is 1 meter wide at scale 1.0
        // viewport height depends on the render aspect ratio
        Cam {
            pos: Vec3::new(0.0, 0.0, -1.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            viewport_dist: scale / 2.0,
            viewport_dims: Vec2::new(scale, scale * 0.6),
        }
    }

    pub fn look_at(&mut self, p: Vec3) {
        self.dir = (p - self.pos).normalize();
        let world_up = Vec3::new(0.0, 1.0, 0.0); // assumption
        self.right = self.dir.cross(world_up).normalize();
        self.up = self.right.cross(self.dir).normalize();
    }
}

pub struct Scene {
    pub scale: f32,
    pub cam: Cam,
    pub lights: Vec<Light>,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl Scene {
    pub fn new(scale: f32, viewport_aspect_ratio: f32) -> Scene {
        Scene {
            scale,
            cam: Cam::new(scale, viewport_aspect_ratio),
            lights: vec![],
            shapes: vec![],
        }
    }
}
