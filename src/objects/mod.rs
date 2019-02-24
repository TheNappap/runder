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

pub trait Object : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>{
        let transformation = self.transformation();
        let transformed_ray = Ray::new(transformation.inverted()*ray.origin(), transformation.inverted()*ray.direction() );
        match self.intersect_without_transformation(&transformed_ray) {
            None => None,
            Some(int) => Some(int.transform(self.transformation(), &ray))
        }
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection>;
    fn transformation(&self) -> &Transformation;
    fn bounding_box(&self) -> &BoundingBox;
    fn material(&self) -> &Material;
}