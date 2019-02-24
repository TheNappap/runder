mod bvh;

use cg_tools::Ray;
use math::Point;
use scene::Intersection;

trait AccelerationStructure {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn visible(&self, from: Point, to: Point) -> bool;
}