
use objects::Object;
use acceleration::AccelerationStructure;
use cg_tools::Ray;
use math::Point;
use scene::Intersection;


enum BoundingVolumeHierarchy {
    Composite(Box<BoundingVolumeHierarchy>,Box<BoundingVolumeHierarchy>),
    Leaf(Vec<Box<Object>>)
}

impl AccelerationStructure for BoundingVolumeHierarchy {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        unimplemented!()
    }

    fn visible(&self, from: Point, to: Point) -> bool {
        unimplemented!()
    }
}