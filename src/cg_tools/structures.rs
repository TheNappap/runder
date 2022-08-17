
extern crate itertools;
use self::itertools::iproduct;

use super::Transformation;
use crate::math::{Point, Direction, Normal, EPSILON};

//////////////////
//Ray
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray{
    origin:Point,
    direction:Direction
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Ray{
        Ray{origin, direction: direction}
    }

    pub fn origin(&self) -> Point { self.origin }
    pub fn direction(&self) -> Direction { self.direction }
}

//////////////////
//BoundingBox
//////////////////
#[derive(Debug)]
pub struct BoundingBox {
    min: Point,
    max: Point
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> BoundingBox{
        BoundingBox{min, max}
    }

    pub fn new_from_origin(corner_point: Point) -> BoundingBox{
        let min = corner_point.min(Point::origin());
        let max = corner_point.max(Point::origin());
        BoundingBox{min, max}
    }

    pub fn min(&self) -> Point { self.min }
    pub fn max(&self) -> Point { self.max }

    pub fn union(&self, bounds: &BoundingBox) -> BoundingBox {
        let min = bounds.min.min(self.min);
        let max = bounds.max.max(self.max);

        BoundingBox{min, max}
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.min.x <= point.x && self.max.x >= point.x
            && self.min.y <= point.y && self.max.y >= point.y
            && self.min.z <= point.z && self.max.z >= point.z
    }

    pub fn transformed(&self, transformation: &Transformation) -> BoundingBox {
        let transform = transformation.matrix();
        let (min, max) = iproduct!(&[self.min.x,self.max.x], &[self.min.y,self.max.y], &[self.min.z,self.max.z]).map(|(x,y,z)|{
            Point::new(*x,*y,*z)
        }).fold((Point::max_point(), Point::min_point()), |(acc_min, acc_max), point|{
            let tf_point = transform*point;
            (acc_min.min(tf_point), acc_max.max(tf_point))
        });
        BoundingBox::new(min, max)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f64, Point, Normal)> {
        let (origin, direction) = (ray.origin(), ray.direction());
        if self.contains(&origin) {
            return Some((0.,origin,Normal::from(*direction.invert())));
        }

        let mut txmin = (self.min.x - origin.x) / direction.x;
        let mut txmax = (self.max.x - origin.x) / direction.x;

        if txmin > txmax { std::mem::swap(&mut txmin, &mut txmax) }
        let mut tmin = txmin;
        let mut tmax = txmax;

        let mut tymin = (self.min.y - origin.y) / direction.y;
        let mut tymax = (self.max.y - origin.y) / direction.y;

        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax) }
        if (tmin > tymax) || (tymin > tmax) { return None; }

        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (self.min.z - origin.z) / direction.z;
        let mut tzmax = (self.max.z - origin.z) / direction.z;

        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax) }
        if (tmin > tzmax) || (tzmin > tmax) { return None; }

        if tzmin > tmin { tmin = tzmin; }
        //if tzmax < tmax { tmax = tzmax; }

        let t = tmin;
        if t <= 0.0 { return None }
        let point = origin + t**direction;
        let normal = match (t, point) {
            (t,p) if t == txmin && p.x.abs() < EPSILON => Normal::new(-1.0, 0.0, 0.0),
            (t,_) if t == txmin => Normal::new(1.0,0.0,0.0),
            (t,p) if t == tymin && p.y.abs() < EPSILON => Normal::new(0.0, -1.0, 0.0),
            (t,_) if t == tymin => Normal::new(0.0,1.0,0.0),
            (t,p) if t == tzmin && p.z.abs() < EPSILON => Normal::new(0.0, 0.0, -1.0),
            (t,_) if t == tzmin => Normal::new(0.0,0.0,1.0),
            _ => Normal::new(0.0,0.0,0.0)
        };
        Some((t,point,normal))
    }
}