use bvh::{
    aabb::{Bounded, AABB},
    bounding_hierarchy::BHShape,
};

use crate::shapes::Shape;

pub struct ShapeBVHNodeWrapper {
    shape: Box<dyn Shape>,
    node_index: usize,
}

impl ShapeBVHNodeWrapper {
    pub fn new(shape: Box<dyn Shape>) -> ShapeBVHNodeWrapper {
        ShapeBVHNodeWrapper {
            shape,
            node_index: 0,
        }
    }

    pub fn get_shape(&self) -> &Box<dyn Shape> {
        &self.shape
    }
}

impl BHShape for ShapeBVHNodeWrapper {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Bounded for ShapeBVHNodeWrapper {
    fn aabb(&self) -> AABB {
        // delegate to  shape's aabb method
        self.shape.aabb()
    }
}

// // Convert your Vec<Box<dyn Shape>> to Vec<ShapeWrapper>
// let shape_wrappers: Vec<ShapeWrapper> = scene.shapes.into_iter().map(|shape| ShapeWrapper {
//     shape,
//     node_index: 0,
// }).collect();

// // Now you can build the BVH with shape_wrappers
// let bvh = BVH::build(&mut shape_wrappers);
