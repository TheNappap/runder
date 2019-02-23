
use super::Material;
use cg_tools::{Ray, Transformation};
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
    fn material(&self) -> &Material;
}