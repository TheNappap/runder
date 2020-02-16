
use super::{Face, Material, Object};
use cg_tools::{BoundingBox, Transformation, Ray};
use math::{Point};
use scene::Intersection;
use acceleration::{self, AccelerationStructure};

//////////////////
//Mesh
//////////////////
pub struct Mesh {
    faces : Box<AccelerationStructure>,
    material : Box<Material>
}

impl Mesh{
    pub fn new(mut faces: Vec<Box<Face>>, material: Box<Material>) -> Mesh{
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

    fn material(&self) -> &Material { self.material.as_ref() }
}