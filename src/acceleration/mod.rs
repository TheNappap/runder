mod bvh;

use cg_tools::Ray;
use math::{Point, Direction};
use scene::Intersection;
use objects::Object;

pub trait AccelerationStructure {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn visible(&self, from: Point, to: Point) -> bool;
}

pub struct BruteForce{
    objects : Vec<Box<Object>>
}

impl BruteForce {
    pub fn new() -> BruteForce { BruteForce{objects: Vec::new()} }
    pub fn add_object(&mut self, object : Box<Object>){
        self.objects.push(object);
    }
}

impl AccelerationStructure for BruteForce {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.objects.iter()
            .map(|o| o.intersect( ray ) )
            .fold(None, Intersection::closest_intersection)
    }

    fn visible(&self, from: Point, to: Point) -> bool {
        let dir = to - from;
        let distance = dir.length();
        let ray = Ray::new(from, Direction::from(dir));

        for object in &self.objects {
            let opt_int = object.intersect( &ray );
            if let Some(intersect) = opt_int {
                let dist = (intersect.point() - from).length();
                if dist < distance {
                    return false;
                }
            }
        }
        true
    }
}