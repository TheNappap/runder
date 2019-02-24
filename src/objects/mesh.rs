
use super::{Face, Material, Object};
use cg_tools::{BoundingBox, Transformation, Ray};
use math::{Point};
use scene::Intersection;
use objects;

//////////////////
//Mesh
//////////////////
pub struct Mesh {
    faces : Vec<Box<Face>>,
    bbox: BoundingBox,
    transformation : Transformation,
    material : Box<Material>
}

impl Mesh{
    pub fn new(faces: Vec<Box<Face>>, transformation : Transformation, material: Box<Material>) -> Mesh{
        let (min,max) = faces.iter().map(|f| f.bounding_box()).fold((Point::max_point(),Point::min_point()), |acc, bbox|{
            (bbox.min().min(acc.0), bbox.max().max(acc.1))
        });
        let bbox = BoundingBox::new(min,max);
        Mesh{faces, bbox, transformation, material}
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
        self.faces.iter().map(|f|{
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
        })
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

    fn material(&self) -> &Material { self.material.as_ref() }
}