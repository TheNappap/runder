
use super::{Face, Material, Object};
use cg_tools::{BoundingBox, Transformation, Ray};
use math::{Point};
use scene::Intersection;
use objects;
use acceleration::{AccelerationStructure, BoundingVolumeHierarchy};

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
    pub fn new(faces: Vec<Box<Object>>, transformation : Transformation, material: Box<Material>) -> Mesh{
        let bbox = faces.iter().map(|f| f.bounding_box())
            .fold(BoundingBox::new(Point::max_point(), Point::min_point()), |acc, bbox|{
            acc.union(&bbox)
        });
        let acc_structure = Box::new(BoundingVolumeHierarchy::new(faces));
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
        /*self.faces.iter().map(|f|{
            f.intersect_without_transformation(ray)
        }).fold(None, |acc, opt_int : Option<Intersection>|{
            match opt_int {
                None => acc,
                Some(int) => if let Some(acc_int) = acc {
                    if int.t() < acc_int.t() { Some(int) }
                        else { Some(acc_int) }
                }
                    else { Some(int) }
            }
        })*/
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

    fn material(&self) -> &Material { self.material.as_ref() }
}