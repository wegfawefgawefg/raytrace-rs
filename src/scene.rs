use bvh::{bvh::BVH, Point3, Vector3};
use glam::{Vec2, Vec3};

use crate::{
    generate::{ProceduralSceneModifier, SceneModifier},
    shape_bvh_node::ShapeBVHNodeWrapper,
    shapes::Shape,
    structures::Light,
}; // Rng trait provides methods for random number generation

#[derive(Clone, Copy)]
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
            // TODO: 0.6 should probably be viewport_aspect_ratio
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
    pub fn new(scale: f32, cam: Cam) -> Scene {
        Scene {
            scale,
            cam,
            lights: vec![],
            shapes: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn optimize(mut self) -> OptimizedScene {
        let mut wrapped_shapes: Vec<ShapeBVHNodeWrapper> = self
            .shapes
            .drain(..)
            .map(ShapeBVHNodeWrapper::new)
            .collect();
        let bvh = BVH::build(&mut wrapped_shapes);

        OptimizedScene {
            scale: self.scale,
            cam: self.cam,
            lights: self.lights.clone(),
            wrapped_shapes,
            bvh,
        }
    }
}

pub struct OptimizedScene {
    pub scale: f32,
    pub cam: Cam,
    pub lights: Vec<Light>,
    wrapped_shapes: Vec<ShapeBVHNodeWrapper>,
    pub bvh: BVH,
}

impl OptimizedScene {
    pub fn raycast(&self, ray: &crate::structures::Ray) -> Vec<&ShapeBVHNodeWrapper> {
        let bvh_ray: bvh::ray::Ray = bvh::ray::Ray::new(
            Point3::new(ray.origin.x, ray.origin.y, ray.origin.z),
            Vector3::new(ray.dir.x, ray.dir.y, ray.dir.z),
        );
        self.bvh.traverse(&bvh_ray, &self.wrapped_shapes)
    }
}

pub struct SceneBuilder {
    pub scale: f32,
    pub cam: Cam,

    pub scene_modifiers: Vec<SceneModifier>,
    pub procedural_scene_modifiers: Vec<ProceduralSceneModifier>,
}

impl SceneBuilder {
    pub fn new(scale: f32, viewport_aspect_ratio: f32) -> SceneBuilder {
        SceneBuilder {
            scale,
            cam: Cam::new(scale, viewport_aspect_ratio),
            scene_modifiers: Vec::new(),
            procedural_scene_modifiers: Vec::new(),
        }
    }

    pub fn add_mod(&mut self, scene_modifier: SceneModifier) {
        self.scene_modifiers.push(scene_modifier);
    }

    pub fn add_proc_mod(&mut self, proc_scene_modifier: ProceduralSceneModifier) {
        self.procedural_scene_modifiers.push(proc_scene_modifier);
    }

    pub fn generate_static(&self) -> Scene {
        self.generate(1, 0)
    }

    pub fn generate(&self, num_frames: u32, frame: u32) -> Scene {
        let mut scene = Scene::new(self.scale, self.cam);

        for pre_scene_builder in self.scene_modifiers.as_slice() {
            pre_scene_builder(&mut scene);
        }

        for psb in self.procedural_scene_modifiers.as_slice() {
            psb(&mut scene, num_frames, frame);
        }

        // let mut shape_wrappers: Vec<ShapeBVHNodeWrapper> = scene
        //     .shapes
        //     .into_iter()
        //     .map(|shape| ShapeBVHNodeWrapper::new(shape))
        //     .collect();
        // let bvh = BVH::build(&mut shape_wrappers);

        // scene.bvh = Some(bvh);
        scene
    }
}
