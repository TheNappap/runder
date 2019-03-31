
use super::{Object, Material, Plane};
use cg_tools::{Transformation, BoundingBox, Ray};
use math::{Point, Normal, Matrix, EPSILON};
use scene::Intersection;

pub trait Face : Object {
    fn double_sided(&self) -> bool;
}

//////////////////
//Triangle
//////////////////
#[derive(Debug)]
pub struct Triangle {
    vertices : [Point; 3],
    double_sided: bool,
    transformation : Transformation,
    bbox: BoundingBox,
    material : Box<Material>
}

impl Triangle{
    pub fn new(vertices : [Point; 3], double_sided: bool, transformation : Transformation, material: Box<Material>) -> Triangle{
        let tf_points = [transformation.matrix()*vertices[0],transformation.matrix()*vertices[1],transformation.matrix()*vertices[2]];
        let min = tf_points[0].min(tf_points[1]).min(tf_points[2]);
        let max = tf_points[0].max(tf_points[1]).max(tf_points[2]);
        let bbox = BoundingBox::new(min,max);
        Triangle{vertices, double_sided, transformation, bbox, material}
    }
}

impl Face for Triangle {
    fn double_sided(&self) -> bool {
        self.double_sided
    }
}

impl Object for Triangle{
    fn as_ref(&self) -> &Object {
        self
    }

    //Möller–Trumbore
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = (*direction).cross(&edge2);
        let det = edge1.dot(&h);
        if (!self.double_sided || det > -EPSILON) && det < EPSILON {
            return None;
        }
        let inv_det = 1.0/det;
        let s = origin - self.vertices[0];
        let u = inv_det * (s.dot(&h));
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(&edge1);
        let v = inv_det * direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inv_det * edge2.dot(&q);
        if t > 0.0 {
            let point = origin + t**direction;
            let mut normal = Normal::from(edge1.cross(&edge2));
            if det < 0. {
                normal = normal.invert();
            }
            let int = Intersection::new(t, point, normal, self.material());
            Some(int)
        }
            else{
                return None;
            }
    }

    fn transformation(&self) -> &Transformation { &self.transformation }

    fn bounding_box(&self) -> &BoundingBox { &self.bbox }

    fn material(&self) -> &Material { self.material.as_ref() }
}

//////////////////
//Rectangle
//////////////////
#[derive(Debug)]
pub struct Rectangle {
    plane : Plane,
    points: [Point; 4],
    bbox: BoundingBox
}

impl Rectangle{
    pub fn unit_square(transformation: Transformation, double_sided: bool, material: Box<Material>) -> Rectangle{
        let points = [Point::origin(), Point::new(1.0,0.0, 0.0), Point::new(1.0,0.0, 1.0), Point::new(0.0,0.0, 1.0)];
        let bbox = Rectangle::bbox(points, transformation.matrix());
        let plane= Plane::new(points[0], Normal::new(0.0, 1.0, 0.0), double_sided, transformation, material);
        Rectangle{ plane , points, bbox}
    }

    pub fn new(points : [Point; 4], double_sided: bool, transformation: Transformation, material: Box<Material>) -> Rectangle{
        let edge1 = points[1] - points[0];
        let edge2 = points[3] - points[0];
        let normal = Normal::from(edge1.cross(&edge2));

        let bbox = Rectangle::bbox(points, transformation.matrix());
        let plane= Plane::new(points[0], normal, double_sided, transformation, material);
        Rectangle{ plane, points, bbox }
    }

    pub fn plane(&self) -> &Plane { &self.plane }
    pub fn points(&self) -> [Point; 4] { self.points }

    fn bbox(points : [Point; 4], tf_matrix: &Matrix) -> BoundingBox {
        let tf_points = [tf_matrix*points[0],tf_matrix*points[1],tf_matrix*points[2],tf_matrix*points[3]];
        let min = tf_points[0].min(tf_points[1]).min(tf_points[2]).min(tf_points[3]);
        let max = tf_points[0].max(tf_points[1]).max(tf_points[2]).max(tf_points[3]);
        BoundingBox::new(min,max)
    }
}

impl Face for Rectangle {
    fn double_sided(&self) -> bool {
        self.plane.double_sided()
    }
}

impl Object for Rectangle {
    fn as_ref(&self) -> &Object {
        self
    }

    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(intersect) = self.plane.intersect_without_transformation(ray){
            let point = intersect.point();
            let identity = Matrix::identiy();
            let bbox = Rectangle::bbox(self.points, &identity);
            let between_x = point.x >= bbox.min().x - EPSILON && point.x <= bbox.max().x + EPSILON;
            let between_y = point.y >= bbox.min().y - EPSILON && point.y <= bbox.max().y + EPSILON;
            let between_z = point.z >= bbox.min().z - EPSILON && point.z <= bbox.max().z + EPSILON;

            if between_x && between_y && between_z {
                return Some(intersect);
            }
        }
        None
    }

    fn transformation(&self) -> &Transformation { self.plane.transformation() }

    fn bounding_box(&self) -> &BoundingBox {&self.bbox }

    fn material(&self) -> &Material { self.plane.material() }
}