
use super::{Object, Material, Plane};
use cg_tools::{Transformation, BoundingBox, Ray};
use math::{Point, Normal, EPSILON};
use scene::Intersection;

pub trait Face : Object {
    fn bounding_box(&self) -> BoundingBox;
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
    material : Box<Material>
}

impl Triangle{
    pub fn new(vertices : [Point; 3], double_sided: bool, transformation : Transformation, material: Box<Material>) -> Triangle{
        Triangle{vertices, double_sided, transformation, material}
    }
}

impl Face for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        let min = self.vertices[0].min(self.vertices[1]).min(self.vertices[2]);
        let max = self.vertices[0].max(self.vertices[1]).max(self.vertices[2]);
        BoundingBox::new([min,max])
    }

    fn double_sided(&self) -> bool {
        self.double_sided
    }
}

impl Object for Triangle{
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

    fn material(&self) -> &Material { self.material.as_ref() }
}

//////////////////
//Rectangle
//////////////////
#[derive(Debug)]
pub struct Rectangle {
    plane : Plane,
    points: [Point; 4]
}

impl Rectangle{
    pub fn unit_square(transformation: Transformation, material: Box<Material>) -> Rectangle{
        let points = [Point::origin(), Point::new(1.0,0.0, 0.0), Point::new(1.0,0.0, 1.0), Point::new(0.0,0.0, 1.0)];
        Rectangle{ plane: Plane::new(points[0], Normal::new(0.0, 1.0, 0.0), true, transformation, material), points}
    }

    pub fn new(vertices : [Point; 4], double_sided: bool, transformation: Transformation, material: Box<Material>) -> Rectangle{
        let edge1 = vertices[1] - vertices[0];
        let edge2 = vertices[3] - vertices[0];
        let normal = Normal::from(edge1.cross(&edge2));

        Rectangle{ plane: Plane::new(vertices[0], normal, double_sided,transformation, material), points: vertices}
    }

    pub fn plane(&self) -> &Plane { &self.plane }
    pub fn points(&self) -> [Point; 4] { self.points }
}

impl Face for Rectangle {
    fn bounding_box(&self) -> BoundingBox {
        let min = self.points[0].min(self.points[2]);
        let max = self.points[0].max(self.points[2]);
        BoundingBox::new([min,max])
    }

    fn double_sided(&self) -> bool {
        self.plane.double_sided()
    }
}

impl Object for Rectangle {
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(intersect) = self.plane.intersect_without_transformation(ray){
            let point = intersect.point();
            let bbox = self.bounding_box();
            let between_x = point.x >= bbox.points()[0].x - EPSILON && point.x <= bbox.points()[1].x + EPSILON;
            let between_y = point.y >= bbox.points()[0].y - EPSILON && point.y <= bbox.points()[1].y + EPSILON;
            let between_z = point.z >= bbox.points()[0].z - EPSILON && point.z <= bbox.points()[1].z + EPSILON;

            if between_x && between_y && between_z {
                return Some(intersect);
            }
        }
        None
    }

    fn transformation(&self) -> &Transformation { self.plane.transformation() }

    fn material(&self) -> &Material { self.plane.material() }
}