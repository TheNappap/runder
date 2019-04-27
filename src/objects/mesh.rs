
use super::{Face, Material, Object};
use cg_tools::{BoundingBox, Transformation, Ray};
use math::{Point};
use scene::Intersection;
use objects;
use acceleration::{self, AccelerationStructure};

//////////////////
//Mesh
//////////////////
pub struct Mesh {
    faces : Box<AccelerationStructure>,
    bbox: BoundingBox,
    transformation : Transformation,
    material : Box<Material>
}

impl Mesh{
    pub fn new(mut faces: Vec<Box<Face>>, transformation : Transformation, material: Box<Material>) -> Mesh{
        let objects: Vec<_> = faces.drain(..).map(|a| a.as_object() ).collect();
        let bbox = objects.iter().map(|f| f.bounding_box())
            .fold(BoundingBox::new(Point::max_point(), Point::min_point()), |acc, bbox|{
            acc.union(&bbox)
        });
        let acc_structure =acceleration::create_acceleration_structure(objects);
        Mesh{faces: acc_structure, bbox, transformation, material}
    }
}

impl Object for Mesh{
    fn as_ref(&self) -> &Object {
        self
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bounding_box().intersect(ray).is_none() {
            None
        }
        else {
            objects::intersect_impl(self, ray)
        }
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        self.faces.intersect(ray)
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

    fn material(&self) -> &Material { self.material.as_ref() }
}