
use objects::Object;
use acceleration::AccelerationStructure;
use cg_tools::{Ray, BoundingBox};
use math::{Point, Direction};
use scene::Intersection;

enum BVHNode {
    Composite(Box<BoundingVolumeHierarchy>,Box<BoundingVolumeHierarchy>),
    Leaf(Vec<Box<Object>>)
}

struct BoundingVolumeHierarchy {
    bbox: BoundingBox,
    node: BVHNode
}

impl BoundingVolumeHierarchy {
    fn intersect_node(&self, ray: &Ray) -> Option<Intersection> {
        match &self.node {
            BVHNode::Composite(bvh1, bvh2) =>{
                let int1 = bvh1.bbox.intersect(&ray);
                let int2 = bvh2.bbox.intersect(&ray);
                match (int1,int2) {
                    (None,None) => None,
                    (Some(_),None) => bvh1.intersect_node(ray),
                    (None,Some(_)) => bvh2.intersect_node(ray),
                    (Some((t1,_,_)),Some((t2,_,_))) => {
                        let (first,second) = if t1 < t2 { (bvh1, bvh2) } else { (bvh2,bvh1) };
                        let int1 = bvh1.intersect_node(ray);
                        if int1.is_some() && int1.unwrap().t() < t2 { return int1; }
                        let int2 = bvh2.intersect_node(ray);
                        Intersection::closest_intersection(int1,int2)
                    }
                }
            },
            BVHNode::Leaf(objects) => {
                objects.iter()
                    .map(|o| o.intersect( ray ) )
                    .fold(None, Intersection::closest_intersection)
            }
        }
    }

    fn visible_node(&self, ray: &Ray, distance: f64) -> bool {
        match &self.node {
            BVHNode::Composite(bvh1, bvh2) =>{
                let int1 = bvh1.bbox.intersect(&ray);
                let int2 = bvh2.bbox.intersect(&ray);
                match (int1,int2) {
                    (None,None) => true,
                    (Some(_),None) => bvh1.visible_node(ray, distance),
                    (None,Some(_)) => bvh2.visible_node(ray, distance),
                    (Some((t1,_,_)),Some((t2,_,_))) => {
                        let (first,second) = if t1 < t2 { (bvh1, bvh2) } else { (bvh2,bvh1) };
                        if !first.visible_node(ray, distance) { return false; }
                        second.visible_node(ray, distance)
                    }
                }
            },
            BVHNode::Leaf(objects) => {
                for object in objects {
                    if let Some(intersect) = object.intersect( &ray ) {
                        let dist = (intersect.point() - ray.origin()).length();
                        if dist < distance {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }
}

impl AccelerationStructure for BoundingVolumeHierarchy {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bbox.intersect(&ray).is_none() {
            return None;
        }

        self.intersect_node(ray)
    }

    fn visible(&self, from: Point, to: Point) -> bool {
        let dir = to - from;
        let distance = dir.length();
        let ray = Ray::new(from, Direction::from(dir));
        if self.bbox.intersect(&ray).is_none() {
            return true;
        }

        self.visible_node(&ray, distance)
    }
}