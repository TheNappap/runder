
use super::{Face, Material, Object};
use crate::cg_tools::{BoundingBox, Transformation, Ray};
use crate::math::{Point};
use crate::scene::Intersection;
use crate::acceleration::{self, AccelerationStructure};

//////////////////
//Mesh
//////////////////
pub struct Mesh {
    faces : Box<dyn AccelerationStructure>,
    material : Box<dyn Material>
}

impl Mesh{
    pub fn new(mut faces: Vec<Box<dyn Face>>, material: Box<dyn Material>) -> Mesh{
        let instances: Vec<_> = faces.drain(..).map(|a| {
            super::Instance::new(a.as_object())
        } ).collect();
        let acc_structure = acceleration::create_acceleration_structure(instances);
        Mesh{faces: acc_structure, material}
    }
}

impl Object for Mesh{
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.faces.intersect(ray)
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        self.faces.bounding_box(transformation)
    }

    fn material(&self) -> &dyn Material { self.material.as_ref() }
}