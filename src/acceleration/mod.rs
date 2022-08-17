mod bvh;

use crate::cg_tools::{BoundingBox, Ray, Transformation};
use crate::math::{Point, Direction};
use crate::scene::Intersection;
use crate::objects::Instance;
use crate::settings;

#[derive(Copy, Clone, Debug)]
pub enum AccelerationStructureKind {
    BruteForce,
    BVH
}

pub trait AccelerationStructure : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn visible(&self, from: Point, to: Point) -> bool;
    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox;
}

pub fn create_acceleration_structure(instances : Vec<Instance>) -> Box<dyn AccelerationStructure> {
    match settings::get().acceleration_structure {
        AccelerationStructureKind::BruteForce => Box::new(BruteForce::new(instances)),
        AccelerationStructureKind::BVH => Box::new(bvh::BoundingVolumeHierarchy::new(instances)),
    }
}

struct BruteForce{
    instances : Vec<Instance>
}

impl BruteForce {
    pub fn new(instances: Vec<Instance>) -> BruteForce { BruteForce{instances} }
}

impl AccelerationStructure for BruteForce {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.instances.iter()
            .map(|inst| inst.intersect( ray ) )
            .fold(None, Intersection::closest_intersection)
    }

    fn visible(&self, from: Point, to: Point) -> bool {
        let dir = to - from;
        let distance = dir.length();
        let ray = Ray::new(from, Direction::from(dir));

        for instance in &self.instances {
            let opt_int = instance.intersect( &ray );
            if let Some(intersect) = opt_int {
                let dist = (intersect.point() - from).length();
                if dist < distance {
                    return false;
                }
            }
        }
        true
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        let bbox = self.instances.iter()
            .map(|f| f.bounding_box().transformed(transformation))
            .fold(BoundingBox::new(Point::max_point(), Point::min_point()), |acc, bbox| {
                acc.union(&bbox)
            });
        bbox
    }
}