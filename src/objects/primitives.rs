
use crate::math::{Point, Vector, Normal, EPSILON};
use crate::cg_tools::{Ray,Transformation,BoundingBox};
use crate::scene::Intersection;
use crate::objects::{Object,Material};

//////////////////
//Sphere
//////////////////
#[derive(Debug)]
pub struct Sphere {
    material : Box<dyn Material>
}

impl Sphere {
    pub fn new(material: Box<dyn Material>) -> Self{
        Sphere { material }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());

        let a = direction.dot(&direction);
        let b = 2.0*direction.dot(&origin);
        let c = origin.dot(&origin) - 1.0;
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
            let point = origin + t**direction;
            let normal = Normal::from(point - Point::origin());
            Some( Intersection::new(t, point, normal, self.material()) )
        }
            else { None }
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        //https://members.loria.fr/SHornus/ellipsoid-bbox.html
        let matrix = transformation.matrix();
        let center = matrix*Point::origin();
        let rows = matrix.rows();
        let norms : Vec<_> = rows.iter().map(|x|{
            (x[0]*x[0]+x[1]*x[1]+x[2]*x[2]).sqrt()
        }).collect();
        let max = center + Vector::new(norms[0],norms[1],norms[2]);
        let min = center + Vector::new(norms[0],norms[1],norms[2]).invert();
        BoundingBox::new(min,max)
    }

    fn material(&self) -> &dyn Material {
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
    double_sided: bool,
    material : Box<dyn Material>
}

impl Plane{
    pub fn new(point : Point, normal: Normal, double_sided: bool, material: Box<dyn Material>) -> Plane{
        Plane{point, normal, double_sided, material}
    }

    pub fn double_sided(&self) -> bool { self.double_sided }
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());

        let nom = direction.invert().dot(&self.normal);
        let denom = (self.point - origin).dot(&self.normal);
        if (!self.double_sided || nom > -EPSILON) && nom < EPSILON {
            None
        }
        else {
            let normal = if nom < 0.0 { self.normal.invert() } else { self.normal };
            let t = -denom/nom;
            if t < 0.0 { return None }
            let point = origin + t**direction;
            let int = Intersection::new(t, point, normal, self.material());
            Some(int)
        }
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        let transform = transformation.matrix();
        BoundingBox::new(transform*Point::min_point(),transform*Point::max_point())
    }

    fn material(&self) -> &dyn Material { self.material.as_ref() }
}

//////////////////
//Box
//////////////////
#[derive(Debug)]
pub struct BoxObject {
    bounds: BoundingBox,
    material : Box<dyn Material>
}

impl BoxObject{
    pub fn new_from_origin(corner_point: Point, material: Box<dyn Material>) -> BoxObject{
        let bounds = BoundingBox::new_from_origin(corner_point);
        BoxObject{bounds, material}
    }
}

impl Object for BoxObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self.bounds.intersect(ray) {
            None => return None,
            Some(int) => Some(Intersection::new(int.0, int.1, int.2, self.material()))
        }
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        self.bounds.transformed(transformation)
    }

    fn material(&self) -> &dyn Material { self.material.as_ref() }
}