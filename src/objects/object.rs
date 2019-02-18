
use super::Material;
use cg_tools::{Ray, Transformation};
use scene::Intersection;
use math::Direction;

pub trait Object : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>{
        let transformation = self.transformation();
        let transformed_ray = Ray::new(transformation.inverted()*ray.origin(), transformation.inverted()*ray.direction() );
        match self.intersect_without_transformation(&transformed_ray) {
            None => None,
            Some(int) => {
                /*let intt = int.transform(self.transformation(), &ray);
                let t = (intt.point() - ray.origin()).length();
                if (t - intt.t()).abs() > 0.00001 {
                    println!("t: {}, diff: {}", t, intt.t());
                }*/
                Some(int.transform(self.transformation(), &ray))
            }
        }
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection>;
    fn transformation(&self) -> &Transformation;
    fn material(&self) -> &Material;
}