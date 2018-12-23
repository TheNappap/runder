
use primitives::Object;
use math::{VectorTrait, Point, Direction, Normal, Ray, Radiance};
use lights::Light;
use material::Material;

pub struct SceneGraph{
    objects : Vec<Box<Object>>,
    lights : Vec<Box<Light>>
}

impl SceneGraph {

    pub fn new() -> SceneGraph{
        SceneGraph{objects: Vec::new(), lights: Vec::new()}
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

    pub fn visible(&self, point: Point, from: Direction) -> bool {
        let distance = from.length();
        let ray = Ray::new(point, from);
        for object in &self.objects {
            let opt_int = object.intersect( &ray );
            if let Some(intersect) = opt_int {
                let dist = (intersect.point() - point).length();
                if dist < distance { return false; }
            }
        }
        true
    }

    pub fn receive_radiance(&self, intersection: Intersection, outgoing: Direction) -> Radiance{
        let mut radiance = Radiance::zero();
        for light in &self.lights {
            let (rad, incoming) =  light.receive_radiance(&intersection.point, &intersection.normal);
            let visible = self.visible(intersection.point + 1.0e-12*intersection.normal(), incoming);
            if !visible { continue; }

            let cos = intersection.normal.dot(&incoming.normalize());
            let rad = if cos <= 0.0 { Radiance::zero() }
                                else { cos*rad.apply_color(intersection.material.brdf(incoming, outgoing)) };
            radiance = radiance + rad;
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