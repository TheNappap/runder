
use math::{Point, Normal};
use cg_tools::{Ray,Transformation,BoundingBox};
use scene::Intersection;
use objects::{Object,Material};

//////////////////
//Sphere
//////////////////
#[derive(Debug)]
pub struct Sphere {
    transformation : Transformation,
    material : Box<Material>
}

impl Sphere {
    pub fn new(transformation: Transformation, material: Box<Material>) -> Self{
        Sphere { transformation, material }
    }
}

impl Object for Sphere {
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());

        let a = direction.base().dot(&direction.base());
        let b = 2.0*direction.base().dot(&origin.base());
        let c = origin.base().dot(&origin.base()) - 1.0;
        let d = b*b - 4.0*a*c;

        let t;
        if d < 0.0 { return None; }
        else if d > 0.0 {
            let t1 = (-b + d.sqrt())/(2.0*a);
            let t2 = (-b - d.sqrt())/(2.0*a);
            t = if t1 <= 0.0 {
                t2
                } else {
                    if t2 <= 0.0 || t1 < t2 {
                        t1
                    }
                else { t2 }
            };
        }
        else {
            t = -b/(2.0*a);
        }

        if t > 0.0 {
            let point = origin + t*direction;
            let normal = (point - Point::origin()).normalize();
            Some( Intersection::new(t, point, normal, self.material()) )
        }
            else { None }
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn material(&self) -> &Material {
        self.material.as_ref()
    }
}

//////////////////
//Plane
//////////////////
#[derive(Debug)]
pub struct Plane {
    point : Point,
    normal: Normal,
    transformation: Transformation,
    material : Box<Material>
}

impl Plane{
    pub fn new(point : Point, normal: Normal, transformation: Transformation, material: Box<Material>) -> Plane{
        Plane{point, normal, transformation, material}
    }
}

impl Object for Plane {
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());

        let nom = direction.invert().base().dot(&self.normal.base());
        let denom = (self.point - origin).base().dot(&self.normal.base());
        if nom <= 0.0{
            None
        }
        else {
            let t = -denom/nom;
            if t == 0.0 { return None }
            let point = origin + t*direction;
            let int = Intersection::new(t, point, self.normal, self.material());
            Some(int)
        }
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn material(&self) -> &Material { self.material.as_ref() }
}

//////////////////
//Box
//////////////////
#[derive(Debug)]
pub struct BoxObject {
    bbox: BoundingBox,
    transformation: Transformation,
    material : Box<Material>
}

impl BoxObject{
    pub fn new(corner_point: Point, transformation: Transformation, material: Box<Material>) -> BoxObject{
        BoxObject{bbox: BoundingBox::new_from_origin(corner_point), transformation, material}
    }
}

impl Object for BoxObject {
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        match self.bbox.intersect(ray) {
            None => return None,
            Some(int) => Some(Intersection::new(int.0, int.1, int.2, self.material()))
        }
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn material(&self) -> &Material { self.material.as_ref() }
}