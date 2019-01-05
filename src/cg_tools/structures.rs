
use math::{VectorTrait, Point, Direction, Normal};

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
        Ray{origin, direction: direction.normalize()}
    }

    pub fn origin(&self) -> Point { self.origin }
    pub fn direction(&self) -> Direction { self.direction }
}

//////////////////
//BoundingBox
//////////////////
#[derive(Debug)]
pub struct BoundingBox {
    points : [Point;2],
}

impl BoundingBox {
    pub fn new(points: [Point;2]) -> BoundingBox{
        BoundingBox{points}
    }

    pub fn new_from_origin(corner_point: Point) -> BoundingBox{
        BoundingBox{points: [Point::origin(), corner_point]}
    }

    pub fn points(&self) -> [Point;2] {
        self.points
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f64, Point, Normal)> {
        let (origin, direction) = (ray.origin(), ray.direction());
        let mut txmin = (self.points[0].x() - origin.x()) / direction.x();
        let mut txmax = (self.points[1].x() - origin.x()) / direction.x();

        if txmin > txmax { std::mem::swap(&mut txmin, &mut txmax) }
        let mut tmin = txmin;
        let mut tmax = txmax;

        let mut tymin = (self.points[0].y() - origin.y()) / direction.y();
        let mut tymax = (self.points[1].y() - origin.y()) / direction.y();

        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax) }
        if (tmin > tymax) || (tymin > tmax) { return None; }

        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (self.points[0].z() - origin.z()) / direction.z();
        let mut tzmax = (self.points[1].z() - origin.z()) / direction.z();

        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax) }
        if (tmin > tzmax) || (tzmin > tmax) { return None; }

        if tzmin > tmin { tmin = tzmin; }
        //if tzmax < tmax { tmax = tzmax; }

        let t = tmin;
        if t <= 0.0 { return None }let point = origin + t*direction;
        let normal = match (t, point) {
            (t,p) if t == txmin && p.x().abs() < 1e-12 => Normal::new(-1.0,0.0,0.0),
            (t,_) if t == txmin => Normal::new(1.0,0.0,0.0),
            (t,p) if t == tymin && p.y().abs() < 1e-12 => Normal::new(0.0,-1.0,0.0),
            (t,_) if t == tymin => Normal::new(0.0,1.0,0.0),
            (t,p) if t == tzmin && p.z().abs() < 1e-12 => Normal::new(0.0,0.0,-1.0),
            (t,_) if t == tzmin => Normal::new(0.0,0.0,1.0),
            _ => Normal::new(0.0,0.0,0.0)
        };
        Some((t,point,normal))
    }
}