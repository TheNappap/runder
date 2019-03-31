
use objects::Object;
use acceleration::AccelerationStructure;
use cg_tools::{Ray, BoundingBox};
use math::{Point, Direction};
use scene::Intersection;

enum Axis {
    XAxis,
    YAxis,
    ZAxis
}

enum BVHNode {
    Composite(Box<BoundingVolumeHierarchy>,Box<BoundingVolumeHierarchy>),
    Leaf(Vec<Box<Object>>)
}

pub struct BoundingVolumeHierarchy {
    bbox: BoundingBox,
    node: BVHNode
}

impl BoundingVolumeHierarchy {

    pub fn new(objects: Vec<Box<Object>>) -> BoundingVolumeHierarchy {
        BoundingVolumeHierarchy::split_objects(objects, &Axis::XAxis)
    }

    fn leaf(objects: Vec<Box<Object>>) -> BoundingVolumeHierarchy{
        let bbox = objects.iter().map(|o|{
            o.bounding_box()
        }).fold(BoundingBox::new(Point::max_point(), Point::min_point()), |acc, bbox|{
            acc.union(&bbox)
        });
        BoundingVolumeHierarchy{bbox, node: BVHNode::Leaf(objects)}
    }

    fn composite(objects1: Vec<Box<Object>>, objects2: Vec<Box<Object>>, split_axis: Axis) -> BoundingVolumeHierarchy{
        let bvh1 = BoundingVolumeHierarchy::split_objects(objects1, &split_axis);
        let bvh2 = BoundingVolumeHierarchy::split_objects(objects2, &split_axis);
        let bbox = bvh1.bbox.union(&bvh2.bbox);
        BoundingVolumeHierarchy{bbox, node: BVHNode::Composite(Box::new(bvh1), Box::new(bvh2))}
    }

    fn split_objects(mut objects: Vec<Box<Object>>, split_axis: &Axis) -> BoundingVolumeHierarchy {
        if objects.len() < 4 {
            return BoundingVolumeHierarchy::leaf(objects);
        }

        objects.sort_by(|a,b|{
            let bbox1 = a.bounding_box();
            let bbox2 = b.bounding_box();
            if let Some(ord) = bbox1.min().x.partial_cmp(&bbox2.min().x) {
                ord
            }
            else { std::cmp::Ordering::Equal }
        });

        let chunk_size = (objects.len() + 1) / 2; //round up
        let (objects1, objects2): (Vec<Box<Object>>,Vec<Box<Object>>) = objects.drain(..).enumerate()
            .fold( (Vec::new(),Vec::new()),|(mut vec1,mut vec2), (i,o)|{
            if i < chunk_size { vec1.push(o); }
            else { vec2.push(o); }
            (vec1,vec2)
        });

        let next_split_axis = match *split_axis {
            Axis::XAxis => Axis::YAxis,
            Axis::YAxis => Axis::ZAxis,
            Axis::ZAxis => Axis::XAxis,
        };
        BoundingVolumeHierarchy::composite(objects1, objects2, next_split_axis)
    }

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