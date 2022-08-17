
use crate::settings;
use crate::math::{Direction, Matrix, Normal, Point, EPSILON};
use crate::cg_tools::{Ray,Transformation,Radiance};
use crate::objects::{Instance, Light, Material};
use crate::acceleration::{self, AccelerationStructure};
use crate::camera::PerspectiveCamera;

pub struct Scene {
    acc_structure: Box<dyn AccelerationStructure>,
    lights : Vec<Box<dyn Light>>,
    camera: PerspectiveCamera
}

impl Scene {
    pub fn new(instances : Vec<Instance>, lights : Vec<Box<dyn Light>>, camera: PerspectiveCamera) -> Scene {
        let acc_structure = acceleration::create_acceleration_structure(instances);
        Scene {acc_structure, lights, camera}
    }

    pub fn camera(&self) -> &PerspectiveCamera {
        &self.camera
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.acc_structure.intersect(ray)
    }

    pub fn visible(&self, from: Point, to: Point) -> bool {
        self.acc_structure.visible(from, to)
    }

    fn normal_rotation(base_normal: Normal) -> Matrix {
        let cos = Normal::up().dot(&base_normal);
        match cos {
            1. => return Matrix::identiy(),
            -1. => return Matrix::identiy()*-1.,
            _ => ()
        }
        let v = Normal::up().cross(&base_normal);
        let vx = Matrix::from([[0., -v.z, v.y],[v.z, 0., -v.x], [-v.y, v.x, 0.]]);
        Matrix::identiy() + vx + vx*vx*(1./(1.+cos))
    }

    fn calc_light_transfer_through_intersection(&self, outgoing: Direction, intersection: &Intersection, from_point: Point, from_normal: Normal) -> Radiance {
        let from_point = from_point + EPSILON**from_normal;
        let to_point = intersection.point() + EPSILON**intersection.normal();
        let diff = from_point - to_point;
        let incoming = Direction::from(diff);

        let visible = self.visible(from_point + EPSILON**from_normal, intersection.point() + EPSILON**intersection.normal());
        if !visible { return Radiance::zero(); }

        let r = diff.length();
        if r == 0. {
            return Radiance::zero();
        }
        let cos_in = intersection.normal().dot(&incoming).max(0.0);
        let cos_out = from_normal.dot(&incoming.invert()).max(0.0);

        let transfer_factor = (cos_in*cos_out)/(r*r);
        let brdf = Radiance::from(intersection.material().brdf(incoming, outgoing));
        transfer_factor*brdf
    }

    pub fn receive_radiance(&self, intersection: Intersection, outgoing: Direction, indirect: bool) -> Radiance{
        //indirect light
        let mut indirect_radiance = Radiance::zero();
        if indirect {
            let branching_factor = 4;
            let rotation = Scene::normal_rotation(intersection.normal());
            for (x,y,z) in settings::get().light_sampling_technique.sample_hemisphere(branching_factor) {
                let direction= Direction::from(*(rotation * Normal::new(x,y,z)));
                match self.intersect(&Ray::new(intersection.point(), direction)) {
                    None => continue,
                    Some(i) => {
                        let rad= self.receive_radiance(i, direction.invert(), false);
                        let transfer_factor = self.calc_light_transfer_through_intersection(outgoing, &intersection, i.point(), i.normal());
                        indirect_radiance = indirect_radiance + rad*transfer_factor;
                    }
                };
            }
            indirect_radiance = indirect_radiance/branching_factor as f64;
        }

        //direct light
        let mut direct_radiance = Radiance::zero();
        for light in &self.lights {
            let mut rad = Radiance::zero();
            let light_points = light.light_points(settings::get().light_sampling_technique);
            let amount = light_points.len();
            for (light_point, opt_normal) in light_points{
                let light_normal = match opt_normal { Some(n) => n, None => Normal::from((light_point - intersection.point()).invert()) };

                let transfer_factor = self.calc_light_transfer_through_intersection(outgoing, &intersection, light_point, light_normal);
                let rad_from_light = light.radiance_from_point(light_point);
                rad = rad + transfer_factor*rad_from_light;
            }

            direct_radiance = direct_radiance + rad/amount as f64;
        }
        direct_radiance + indirect_radiance
    }

}

#[derive(Copy, Clone)]
pub struct Intersection<'a>{
    t : f64,
    point : Point,
    normal : Normal,
    material: &'a dyn Material
}

impl<'a> Intersection<'a>{
    pub fn new(t : f64, point : Point, normal : Normal, material: &dyn Material) -> Intersection{
        Intersection{t, point, normal, material}
    }

    pub fn t(&self) -> f64 { self.t }
    pub fn point(&self) -> Point { self.point }
    pub fn normal(&self) -> Normal { self.normal }
    pub fn material(&self) -> &'a dyn Material { self.material }

    pub fn transform(mut self, transformation: &Transformation, ray: &Ray) -> Intersection<'a> {
        self.point = transformation.matrix()*self.point;
        self.t = (self.point - ray.origin()).length();
        self.normal = Normal::from( transformation.inverted().transpose()**self.normal );
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