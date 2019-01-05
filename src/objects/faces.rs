
use super::{Object, Material, Plane};
use cg_tools::{Transformation, BoundingBox, Ray};
use math::{VectorTrait, Point, Normal};
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
    transformation : Transformation,
    material : Box<Material>
}

impl Triangle{
    pub fn new(vertices : [Point; 3], transformation : Transformation, material: Box<Material>) -> Triangle{
        Triangle{vertices, transformation, material}
    }
}

impl Face for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        let min = self.vertices[0].min(self.vertices[1]).min(self.vertices[2]);
        let max = self.vertices[0].max(self.vertices[1]).max(self.vertices[2]);
        BoundingBox::new([min,max])
    }

    fn double_sided(&self) -> bool {
        unimplemented!()
    }
}

impl Object for Triangle{
    //Möller–Trumbore
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = direction.cross(edge2);
        let a = edge1.dot(&h);
        if a > -1e-12 && a < 1e-12{
            return None;
        }    // This ray is parallel to this triangle.
        let f = 1.0/a;
        let s = origin - self.vertices[0];
        let u = f * (s.dot(&h));
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);
        if t > 0.0 { // ray intersection
            let point = origin + t*direction;
            let normal = (edge1.cross(edge2)).normalize();
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
        Rectangle{ plane: Plane::new(points[0], Normal::new(0.0, 1.0, 0.0), transformation, material), points}
    }

    pub fn new(vertices : [Point; 4], transformation: Transformation, material: Box<Material>) -> Rectangle{
        let edge1 = vertices[1] - vertices[0];
        let edge2 = vertices[3] - vertices[0];
        let normal = edge1.cross(edge2).normalize();

        Rectangle{ plane: Plane::new(vertices[0], normal, transformation, material), points: vertices}
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
        unimplemented!()
    }
}

impl Object for Rectangle {
    fn intersect_without_transformation(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(intersect) = self.plane.intersect_without_transformation(ray){
            let point = intersect.point();
            let bbox = self.bounding_box();
            let between_x = point.x() >= bbox.points()[0].x() - 1e-12 && point.x() <= bbox.points()[1].x() + 1e-12;
            let between_y = point.y() >= bbox.points()[0].y() - 1e-12 && point.y() <= bbox.points()[1].y() + 1e-12;
            let between_z = point.z() >= bbox.points()[0].z() - 1e-12 && point.z() <= bbox.points()[1].z() + 1e-12;

            if between_x && between_y && between_z {
                return Some(intersect);
            }
        }
        None
    }

    fn transformation(&self) -> &Transformation { self.plane.transformation() }

    fn material(&self) -> &Material { self.plane.material() }
}