use glam::Vec3;

pub trait Material: Sync {
    fn color_at(&self, hit_pos: &Vec3) -> Vec3;
    fn ambient_at(&self, hit_pos: &Vec3) -> f32;
    fn diffuse_at(&self, hit_pos: &Vec3) -> f32;
    fn specular_at(&self, hit_pos: &Vec3) -> f32;
    fn reflective_at(&self, hit_pos: &Vec3) -> f32;
}

#[derive(Clone)]
pub struct BasicMaterial {
    pub color: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub reflective: f32,
}

impl BasicMaterial {
    pub fn new(
        color: Vec3,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        reflective: f32,
    ) -> BasicMaterial {
        BasicMaterial {
            color,
            ambient,
            diffuse,
            specular,
            reflective,
        }
    }
}

impl Material for BasicMaterial {
    fn color_at(&self, _hit_pos: &Vec3) -> Vec3 {
        self.color
    }

    fn ambient_at(&self, _hit_pos: &Vec3) -> f32 {
        self.ambient
    }

    fn diffuse_at(&self, _hit_pos: &Vec3) -> f32 {
        self.diffuse
    }

    fn specular_at(&self, _hit_pos: &Vec3) -> f32 {
        self.specular
    }

    fn reflective_at(&self, _hit_pos: &Vec3) -> f32 {
        self.reflective
    }
}

#[derive(Clone)]
pub struct CheckerMaterial {
    pub color1: Vec3,
    pub color2: Vec3,
    pub scale: f32,
    pub basic_material: BasicMaterial,
}

impl CheckerMaterial {
    pub fn new(
        color1: Vec3,
        color2: Vec3,
        scale: f32,
        basic_material: BasicMaterial,
    ) -> CheckerMaterial {
        CheckerMaterial {
            color1,
            color2,
            scale,
            basic_material,
        }
    }

    fn checker_at(&self, hit_pos: &Vec3) -> Vec3 {
        let s = self.scale;
        let pattern = (((hit_pos.x / s).floor() + (hit_pos.y / s).floor() + (hit_pos.z / s).floor())
            as i32)
            % 2;
        if pattern == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}

impl Material for CheckerMaterial {
    fn color_at(&self, hit_pos: &Vec3) -> Vec3 {
        self.checker_at(hit_pos)
    }

    fn ambient_at(&self, hit_pos: &Vec3) -> f32 {
        self.basic_material.ambient_at(hit_pos)
    }

    fn diffuse_at(&self, hit_pos: &Vec3) -> f32 {
        self.basic_material.diffuse_at(hit_pos)
    }

    fn specular_at(&self, hit_pos: &Vec3) -> f32 {
        self.basic_material.specular_at(hit_pos)
    }

    fn reflective_at(&self, hit_pos: &Vec3) -> f32 {
        self.basic_material.reflective_at(hit_pos)
    }
}
