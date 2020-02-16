
use objects::Instance;
use acceleration::AccelerationStructure;
use cg_tools::{Ray, BoundingBox, Transformation};
use math::{Point, Direction};
use scene::Intersection;

enum Axis {
    XAxis,
    YAxis,
    ZAxis
}

enum BVHNode {
    Composite(Box<BoundingVolumeHierarchy>,Box<BoundingVolumeHierarchy>),
    Leaf(Vec<Instance>)
}

impl BVHNode {
    fn leaf(instances: Vec<Instance>) -> BVHNode{
        BVHNode::Leaf(instances)
    }

    fn composite(instances1: Vec<Instance>, instances2: Vec<Instance>, split_axis: Axis) -> BVHNode{
        let bvh1 = BoundingVolumeHierarchy::new(instances1);
        let bvh2 = BoundingVolumeHierarchy::new(instances2);
        BVHNode::Composite(Box::new(bvh1), Box::new(bvh2))
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        match self {
            BVHNode::Leaf(instances) => {
                instances.iter().map(|o|{
                    o.bounding_box().transformed(transformation)
                }).fold(BoundingBox::new(Point::max_point(), Point::min_point()), |acc, bbox|{
                    acc.union(&bbox)
                })
            },
            BVHNode::Composite(bvh1, bvh2) => {
                let bbox1 = bvh1.bounding_box(transformation);
                let bbox2 = bvh2.bounding_box(transformation);
                bbox1.union(&bbox2)
            }
        }
    }
}

pub struct BoundingVolumeHierarchy {
    bbox: BoundingBox,
    root: BVHNode
}

impl BoundingVolumeHierarchy {

    pub fn new(instances: Vec<Instance>) -> BoundingVolumeHierarchy {
        let root = BoundingVolumeHierarchy::split_instances(instances, &Axis::XAxis);
        let bbox = root.bounding_box(&Transformation::new());
        BoundingVolumeHierarchy{bbox, root}
    }

    fn split_instances(mut instances: Vec<Instance>, split_axis: &Axis) -> BVHNode {
        if instances.len() < 4 {
            return BVHNode::leaf(instances);
        }

        instances.sort_by(|a,b|{
            let bbox1 = a.bounding_box();
            let bbox2 = b.bounding_box();
            if let Some(ord) = bbox1.min().x.partial_cmp(&bbox2.min().x) {
                ord
            }
            else { std::cmp::Ordering::Equal }
        });

        let chunk_size = (instances.len() + 1) / 2; //round up
        let (instances1, instances2): (Vec<Instance>,Vec<Instance>) = instances.drain(..).enumerate()
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
        BVHNode::composite(instances1, instances2, next_split_axis)
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bbox.intersect(&ray).is_none() {
            return None;
        }

        match &self.root {
            BVHNode::Composite(bvh1, bvh2) =>{
                let int1 = bvh1.bbox.intersect(&ray);
                let int2 = bvh2.bbox.intersect(&ray);
                match (int1,int2) {
                    (None,None) => None,
                    (Some(_),None) => bvh1.intersect(ray),
                    (None,Some(_)) => bvh2.intersect(ray),
                    (Some((t1,_,_)),Some((t2,_,_))) => {
                        let (first,second) = if t1 < t2 { (bvh1, bvh2) } else { (bvh2,bvh1) };
                        let int1 = bvh1.intersect(ray);
                        if int1.is_some() && int1.unwrap().t() < t2 { return int1; }
                        let int2 = bvh2.intersect(ray);
                        Intersection::closest_intersection(int1,int2)
                    }
                }
            },
            BVHNode::Leaf(instances) => {
                instances.iter()
                    .map(|o| o.intersect( ray ) )
                    .fold(None, Intersection::closest_intersection)
            }
        }
    }

    fn visible(&self, ray: &Ray, distance: f64) -> bool {
        if self.bbox.intersect(&ray).is_none() {
            return false;
        }

        match &self.root {
            BVHNode::Composite(bvh1, bvh2) =>{
                let int1 = bvh1.bbox.intersect(&ray);
                let int2 = bvh2.bbox.intersect(&ray);
                match (int1,int2) {
                    (None,None) => true,
                    (Some(_),None) => bvh1.visible(ray, distance),
                    (None,Some(_)) => bvh2.visible(ray, distance),
                    (Some((t1,_,_)),Some((t2,_,_))) => {
                        let (first,second) = if t1 < t2 { (bvh1, bvh2) } else { (bvh2,bvh1) };
                        if !first.visible(ray, distance) { return false; }
                        second.visible(ray, distance)
                    }
                }
            },
            BVHNode::Leaf(instances) => {
                for object in instances {
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
        //println!("{:?}",self.bbox);
        if self.bbox.intersect(&ray).is_none() {
            return None;
        }

        self.intersect(ray)
    }

    fn visible(&self, from: Point, to: Point) -> bool {
        let dir = to - from;
        let distance = dir.length();
        let ray = Ray::new(from, Direction::from(dir));
        if self.bbox.intersect(&ray).is_none() {
            return true;
        }

        self.visible(&ray, distance)
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        self.root.bounding_box(transformation)
    }
}