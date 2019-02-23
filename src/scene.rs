
use std::sync::{Arc};
use settings::Settings;

use math::{Point, Vector, Direction, Normal, EPSILON};
use cg_tools::{Ray,Transformation};
use objects::{Object, Light, Material};
use units::Radiance;

pub struct SceneGraph{
    settings: Arc<Settings>,
    objects : Vec<Box<Object>>,
    lights : Vec<Box<Light>>
}

impl SceneGraph {

    pub fn new(settings: Arc<Settings>) -> SceneGraph{
        SceneGraph{settings, objects: Vec::new(), lights: Vec::new()}
    }

    pub fn add_object(&mut self, object : Box<Object>){
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light : Box<Light>){
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.objects.iter()
            .map(|o| o.intersect( ray ) )
            .fold(None, Intersection::closest_intersection)
    }

    pub fn visible(&self, from: Point, to: Point) -> bool {
        let dir = to - from;
        let distance = dir.length();
        let ray = Ray::new(from, Direction::from(dir));
        for object in &self.objects {
            let opt_int = object.intersect( &ray );
            if let Some(intersect) = opt_int {
                let dist = (intersect.point() - from).length();
                if dist < distance {
                    return false;
                }
            }
        }
        true
    }

    pub fn receive_radiance(&self, intersection: Intersection, outgoing: Direction) -> Radiance{
        let mut radiance = Radiance::zero();
        for light in &self.lights {
            let mut rad = Radiance::zero();
            let light_points = light.light_points(self.settings.light_sampling_technique);
            let amount = light_points.len();
            for (light_point, opt_normal) in light_points{
                let incoming = Direction::from(light_point - intersection.point());
                let light_normal = match opt_normal { Some(n) => n, None => Normal::from(*incoming.invert()) };

                let visible = self.visible(light_point + EPSILON**light_normal, intersection.point() + EPSILON**intersection.normal());
                if !visible { continue }

                let r = (light_point - intersection.point()).length();
                let cos_point = intersection.normal().dot(&incoming).max(0.0);
                let cos_light = light_normal.dot(&incoming.invert()).max(0.0);

                let factor = (cos_point*cos_light)/(r*r);
                let rad_from_light = light.radiance_from_point(light_point);
                let brdf = intersection.material.brdf(incoming, outgoing);
                rad = rad + factor*rad_from_light.apply_color(brdf);
            }

            radiance = radiance + rad*(1.0/amount as f64);
        }
        radiance
    }

}

#[derive(Copy, Clone)]
pub struct Intersection<'a>{
    t : f64,
    point : Point,
    normal : Normal,
    material: &'a Material
}

impl<'a> Intersection<'a>{
    pub fn new(t : f64, point : Point, normal : Normal, material: &Material) -> Intersection{
        Intersection{t, point, normal, material}
    }

    pub fn t(&self) -> f64 { self.t }
    pub fn point(&self) -> Point { self.point }
    pub fn normal(&self) -> Normal { self.normal }
    pub fn material(&self) -> &'a Material { self.material }

    pub fn transform(mut self, transformation: &Transformation, ray: &Ray) -> Intersection<'a> {
        self.point = transformation.matrix()*self.point;
        self.t = (self.point - ray.origin()).length();
        self.normal = Normal::from( &transformation.inverted().transpose()**self.normal );
        self
    }

    pub fn closest_intersection(first: Option<Intersection<'a>>, second: Option<Intersection<'a>>) -> Option<Intersection<'a>>{
        if let Some(int1) = first {
            if let Some(int2) = second {
                if int1.t < int2.t { Some(int1) }
                else { Some(int2) }
            }
            else { Some(int1) }
        } else { second }
    }
}