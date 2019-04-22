mod faces;
mod lights;
mod materials;
mod mesh;
mod obj_import;
mod primitives;

pub use self::faces::{Face, Triangle, Rectangle};
pub use self::lights::{Light,PointLight,SurfaceLight};
pub use self::materials::{Material,Lambertian};
pub use self::mesh::{Mesh};
pub use self::obj_import::{parse_obj};
pub use self::primitives::*;

use cg_tools::{Ray, Transformation, BoundingBox};
use scene::Intersection;
use statistics;

pub trait Object : Send + Sync {
    fn as_ref(&self) -> &Object;

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        intersect_impl(self.as_ref(), &ray)
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection>;
    fn transformation(&self) -> &Transformation;
    fn bounding_box(&self) -> &BoundingBox;
    fn material(&self) -> &Material;
}

fn intersect_impl<'a,'b>(object: &'a Object, ray: &'b Ray) -> Option<Intersection<'a>> {
    let transformation = object.transformation();
    let transformed_ray = Ray::new(transformation.inverted()*ray.origin(), transformation.inverted()*ray.direction() );
    match object.intersect_without_transformation(&transformed_ray) {
        None => {
            statistics::object_intersection(false);
            None
        },
        Some(int) => {
            statistics::object_intersection(true);
            Some(int.transform(object.transformation(), &ray))
        }
    }
}