extern crate itertools;

use self::itertools::iproduct;

use math::{Point, Vector, Normal, Matrix, EPSILON};
use cg_tools::{Ray,Transformation,BoundingBox};
use scene::Intersection;
use objects::{Object,Material};

//////////////////
//Sphere
//////////////////
#[derive(Debug)]
pub struct Sphere {
    transformation : Transformation,
    bbox: BoundingBox,
    material : Box<Material>
}

impl Sphere {
    pub fn new(transformation: Transformation, material: Box<Material>) -> Self{
        let bbox = Sphere::bbox(&transformation);
        Sphere { transformation, bbox, material }
    }

    fn bbox(transformation: &Transformation) -> BoundingBox {
        //https://members.loria.fr/SHornus/ellipsoid-bbox.html
        //https://math.stackexchange.com/questions/1254181/bounding-box-of-ellipsoid
        let center = transformation.matrix()*Point::origin();
        let x = transformation.inverted().get(0,0).sqrt();
        let y = transformation.inverted().get(1,1).sqrt();
        let z = transformation.inverted().get(2,2).sqrt();
        let max = center + Vector::new(x,y,z);
        let min = center + Vector::new(x,y,z).invert();
        BoundingBox::new(min,max)
    }
}

impl Object for Sphere {
    fn as_ref(&self) -> &Object {
        self
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
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

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

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
    double_sided: bool,
    transformation: Transformation,
    bbox: BoundingBox,
    material : Box<Material>
}

impl Plane{
    pub fn new(point : Point, normal: Normal, double_sided: bool, transformation: Transformation, material: Box<Material>) -> Plane{
        let bbox = BoundingBox::new(Point::min_point(),Point::max_point());
        Plane{point, normal, double_sided, transformation, bbox, material}
    }

    pub fn double_sided(&self) -> bool { self.double_sided }
}

impl Object for Plane {
    fn as_ref(&self) -> &Object {
        self
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
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

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

    fn material(&self) -> &Material { self.material.as_ref() }
}

//////////////////
//Box
//////////////////
#[derive(Debug)]
pub struct BoxObject {
    bounds_without_transform: BoundingBox,
    transformation: Transformation,
    bbox:  BoundingBox,
    material : Box<Material>
}

impl BoxObject{
    pub fn new_from_origin(corner_point: Point, transformation: Transformation, material: Box<Material>) -> BoxObject{
        let bounds_without_transform = BoundingBox::new_from_origin(corner_point);
        let bbox = BoxObject::bbox(&bounds_without_transform, transformation.matrix());
        BoxObject{bounds_without_transform, transformation, bbox, material}
    }

    fn bbox(bounds: &BoundingBox, tf_matrix: &Matrix) -> BoundingBox {
        let min = bounds.min();
        let max = bounds.max();
        let (min, max) = iproduct!(&[min.x,max.x], &[min.y,max.y], &[min.z,max.z]).map(|(x,y,z)|{
            Point::new(*x,*y,*z)
        }).fold((Point::max_point(), Point::min_point()), |(acc_min, acc_max), point|{
            let tf_point = tf_matrix*point;
            (acc_min.min(tf_point), acc_max.max(tf_point))
        });
        BoundingBox::new(min, max)
    }
}

impl Object for BoxObject {
    fn as_ref(&self) -> &Object {
        self
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        match self.bounds_without_transform.intersect(ray) {
            None => return None,
            Some(int) => Some(Intersection::new(int.0, int.1, int.2, self.material()))
        }
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

    fn material(&self) -> &Material { self.material.as_ref() }
}