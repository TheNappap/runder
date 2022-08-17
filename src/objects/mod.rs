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

use std::sync::Arc;
use crate::math::Direction;
use crate::cg_tools::{Ray, Transformation, BoundingBox, Color};
use crate::scene::Intersection;
use crate::statistics;
use crate::settings;

//Object
pub trait Object : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox;
    fn material(&self) -> &dyn Material;
}

//Instance
pub struct Instance {
    object: Arc<dyn Object>,
    transformation: Transformation,
    bbox: BoundingBox
}

impl Instance {
    pub fn new(object: Arc<dyn Object>) -> Instance {
        let transformation = Transformation::new();
        let bbox = object.bounding_box(&transformation);
        Instance{object, transformation, bbox}
    }

    pub fn transformed(object: Arc<dyn Object>, transformation: Transformation) -> Instance {
        let bbox = object.bounding_box(&transformation);
        Instance{object, transformation, bbox}
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let transformed_ray = Ray::new(self.transformation.inverted()*ray.origin(), self.transformation.inverted()*ray.direction() );
        let intersect = match settings::get().render_mode {
            settings::RenderMode::BoundingBox => {
                let int = self.bbox.intersect(&ray);
                match int {
                    None => { None},
                    Some((t, point, normal)) => {Some(Intersection::new(t, point, normal, &BBOX_MATERIAL))},
                }
            },
            _ => self.object.intersect(&transformed_ray)
        };
        match intersect {
            None => {
                statistics::object_intersection(false);
                None
            },
            Some(int) => {
                statistics::object_intersection(true);
                Some(int.transform(&self.transformation, &ray))
            }
        }
    }

    pub fn transformation(&self) -> &Transformation{
        &self.transformation
    }

    pub fn bounding_box(&self) -> &BoundingBox{
        &self.bbox
    }

    pub fn material(&self) -> &dyn Material{
        self.object.material()
    }
}

#[derive(Clone, Debug)]
struct BoundingBoxMaterial;
static BBOX_MATERIAL: BoundingBoxMaterial = BoundingBoxMaterial;

impl Material for BoundingBoxMaterial {
    fn brdf(&self, _: Direction, _: Direction) -> Color {
        Color::RGB {r:1.,g:1.,b:1.}
    }
}
