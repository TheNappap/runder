
use math::{VectorTrait, Point, Normal, Ray, Transformation};
use scene::Intersection;
use material::Material;

pub trait Object : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn transformation(&self) -> &Transformation;
    fn material(&self) -> &Material;
}

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
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let center = Point::origin();
        let (origin, direction) = (*self.transformation.inverted()*ray.origin(), *self.transformation.inverted()*ray.direction());

        let moved_origin = origin - center;
        let a = direction.dot(&direction);
        let b = 2.0*direction.dot(&moved_origin);
        let c = moved_origin.dot(&moved_origin) - 1.0;
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
            let normal = (self.transformation.inverted().transpose()*(point - center)).normalize();
            let point = *self.transformation.matrix()*point;
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
    transformation : Transformation,
    material : Box<Material>
}

impl Plane{
    pub fn new(transformation: Transformation, material: Box<Material>) -> Plane{
        Plane{transformation, material}
    }
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let point_in_plane = Point::origin();
        let normal = Normal::new(0.0,1.0,0.0);
        let (origin, direction) = (*self.transformation.inverted()*ray.origin(), *self.transformation.inverted()*ray.direction());

        let nom = direction.invert().dot(&normal);
        let denom = (point_in_plane - origin).dot(&normal);
        if nom <= 0.0{
            None
        }
        else {
            let t = -denom/nom;
            if t == 0.0 { return None }
            let point = origin + t*direction;
            let normal = (self.transformation.inverted().transpose()*normal).normalize();
            let point = *self.transformation.matrix()*point;
            let int = Intersection::new(t, point, normal, self.material());
            Some(int)
        }
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn material(&self) -> &Material { self.material.as_ref() }
}

//////////////////
//Rectangle
//////////////////
#[derive(Debug)]
pub struct Rectangle {
    plane : Plane,
    corner_point: Point
}

impl Rectangle{
    pub fn new(corner_point: Point, transformation: Transformation, material: Box<Material>) -> Rectangle{
        Rectangle{ plane: Plane{transformation, material}, corner_point}
    }

    pub fn corner_point(&self) -> Point { self.corner_point }
}

impl Object for Rectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(intersect) = self.plane.intersect(ray){
            let point = *self.transformation().inverted()*intersect.point();

            if point.x() >= 0.0 && point.z() >= 0.0 && point.x() <= self.corner_point.x() && point.z() <= self.corner_point.z() {
                return Some(intersect);
            }
        }
        None
    }

    fn transformation(&self) -> &Transformation { self.plane.transformation() }

    fn material(&self) -> &Material { self.plane.material() }
}

//////////////////
//Box
//////////////////
#[derive(Debug)]
pub struct BoxObject {
    corner_point: Point,
    transformation : Transformation,
    material : Box<Material>
}

impl BoxObject{
    pub fn new(corner_point: Point, transformation: Transformation, material: Box<Material>) -> BoxObject{
        BoxObject{corner_point, transformation, material}
    }
}

impl Object for BoxObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (*self.transformation.inverted()*ray.origin(), *self.transformation.inverted()*ray.direction());
        let mut txmin = (0.0 - origin.x()) / direction.x();
        let mut txmax = (self.corner_point.x() - origin.x()) / direction.x();

        if txmin > txmax { std::mem::swap(&mut txmin, &mut txmax) }
        let mut tmin = txmin;
        let mut tmax = txmax;

        let mut tymin = (0.0 - origin.y()) / direction.y();
        let mut tymax = (self.corner_point.y() - origin.y()) / direction.y();

        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax) }
        if (tmin > tymax) || (tymin > tmax) { return None; }

        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (0.0 - origin.z()) / direction.z();
        let mut tzmax = (self.corner_point.z() - origin.z()) / direction.z();

        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax) }
        if (tmin > tzmax) || (tzmin > tmax) { return None; }

        if tzmin > tmin { tmin = tzmin; }
        //if tzmax < tmax { tmax = tzmax; }

        let t = tmin;
        if t <= 0.0 { return None }
        let point = origin + t*direction;
        let normal = match (tmin, point) {
            (t,p) if t == txmin && p.x() < 1e-12 => Normal::new(-1.0,0.0,0.0),
            (t,_) if t == txmin => Normal::new(1.0,0.0,0.0),
            (t,p) if t == tymin && p.y() < 1e-12 => Normal::new(0.0,-1.0,0.0),
            (t,_) if t == tymin => Normal::new(0.0,1.0,0.0),
            (t,p) if t == tzmin && p.z() < 1e-12 => Normal::new(0.0,0.0,-1.0),
            (t,_) if t == tzmin => Normal::new(0.0,0.0,1.0),
            _ => Normal::new(0.0,0.0,0.0)
        };
        let normal = (self.transformation.inverted().transpose()*normal).normalize();
        let point = *self.transformation.matrix()*point;
        let int = Intersection::new(t, point, normal, self.material());
        Some(int)
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn material(&self) -> &Material { self.material.as_ref() }
}