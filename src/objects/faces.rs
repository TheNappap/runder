
use std::sync::Arc;

use super::{Object, Material, Plane};
use crate::cg_tools::{Transformation, BoundingBox, Ray};
use crate::math::{Point, Normal, EPSILON};
use crate::scene::Intersection;
use crate::statistics;

pub trait Face : Object {
    fn as_object(self: Box<Self>) -> Arc<dyn Object>;
    fn double_sided(&self) -> bool;
}

//////////////////
//Triangle
//////////////////
#[derive(Debug)]
pub struct Triangle {
    vertices : [Point; 3],
    double_sided: bool,
    material : Box<dyn Material>
}

impl Triangle{
    pub fn new(vertices : [Point; 3], double_sided: bool, material: Box<dyn Material>) -> Triangle{
        Triangle{vertices, double_sided, material}
    }

    fn moller_trumbore(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = (*direction).cross(&edge2);
        let det = edge1.dot(&h);
        if (!self.double_sided || det > -EPSILON) && det < EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
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
            let point = origin + t * *direction;
            let mut normal = Normal::from(edge1.cross(&edge2));
            if det < 0. {
                normal = normal.invert();
            }
            let int = Intersection::new(t, point, normal, self.material());
            Some(int)
        } else {
            return None;
        }
    }
}

impl Face for Triangle {
    fn as_object(self: Box<Self>) -> Arc<dyn Object> { Arc::new(*self) }

    fn double_sided(&self) -> bool {
        self.double_sided
    }
}

impl Object for Triangle{
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let intersect = self.moller_trumbore(ray);
        if intersect.is_none() {
            statistics::triangle_intersection(false);
        } else {
            statistics::triangle_intersection(true);
        }
        intersect
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        let transform = transformation.matrix();
        let tf_points = [transform*self.vertices[0],transform*self.vertices[1],transform*self.vertices[2]];
        let min = tf_points[0].min(tf_points[1]).min(tf_points[2]);
        let max = tf_points[0].max(tf_points[1]).max(tf_points[2]);
        BoundingBox::new(min,max)
    }

    fn material(&self) -> &dyn Material { self.material.as_ref() }
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
    pub fn unit_square(double_sided: bool, material: Box<dyn Material>) -> Rectangle{
        let points = [Point::origin(), Point::new(1.0,0.0, 0.0), Point::new(1.0,0.0, 1.0), Point::new(0.0,0.0, 1.0)];
        let plane= Plane::new(points[0], Normal::new(0.0, 1.0, 0.0), double_sided, material);
        Rectangle{ plane , points }
    }

    pub fn new(points : [Point; 4], double_sided: bool, material: Box<dyn Material>) -> Rectangle{
        let edge1 = points[1] - points[0];
        let edge2 = points[3] - points[0];
        let normal = Normal::from(edge1.cross(&edge2));

        let plane= Plane::new(points[0], normal, double_sided, material);
        Rectangle{ plane, points }
    }

    pub fn plane(&self) -> &Plane { &self.plane }
    pub fn points(&self) -> [Point; 4] { self.points }
}

impl Face for Rectangle {
    fn as_object(self: Box<Self>) -> Arc<dyn Object> { Arc::new(*self) }

    fn double_sided(&self) -> bool {
        self.plane.double_sided()
    }
}

impl Object for Rectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(intersect) = self.plane.intersect(ray){
            let point = intersect.point();
            let bbox = self.bounding_box(&Transformation::new());
            let between_x = point.x >= bbox.min().x - EPSILON && point.x <= bbox.max().x + EPSILON;
            let between_y = point.y >= bbox.min().y - EPSILON && point.y <= bbox.max().y + EPSILON;
            let between_z = point.z >= bbox.min().z - EPSILON && point.z <= bbox.max().z + EPSILON;

            if between_x && between_y && between_z {
                return Some(intersect);
            }
        }
        None
    }

    fn bounding_box(&self, transformation: &Transformation) -> BoundingBox {
        let transform = transformation.matrix();
        let tf_points = [transform*self.points[0],transform*self.points[1],transform*self.points[2],transform*self.points[3]];
        let min = tf_points[0].min(tf_points[1]).min(tf_points[2]).min(tf_points[3]);
        let max = tf_points[0].max(tf_points[1]).max(tf_points[2]).max(tf_points[3]);
        BoundingBox::new(min,max)
    }

    fn material(&self) -> &dyn Material { self.plane.material() }
}