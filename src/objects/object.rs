
use super::Material;
use cg_tools::{Ray, Transformation};
use scene::Intersection;
use math::Direction;

pub trait Object : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>{
        let transformation = self.transformation();
        let ray = Ray::new(transformation.inverted()*ray.origin(), transformation.inverted()*ray.direction() );
        match self.intersect_without_transformation(&ray) {
            None => None,
            Some(int) => Some(int.transform(self.transformation()))
        }
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection>;
    fn transformation(&self) -> &Transformation;
    fn material(&self) -> &Material;
}