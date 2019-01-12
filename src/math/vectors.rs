
use std::ops::{Add,Sub,Mul};
use std::f64;

use super::base_vector::BaseVector;

//////////////////
//Point
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(BaseVector);

impl Point {
    pub fn from_base(base: BaseVector) -> Point
    {
        Point( base )
    }
    pub fn origin() -> Point { Point( BaseVector::new(0.0, 0.0,0.0 ) ) }
    pub fn new(x: f64, y: f64, z: f64) -> Point
    {
        Point( BaseVector::new( x, y, z ) )
    }
    pub fn max_point() -> Point { Point(BaseVector::from_value(f64::MAX)) }
    pub fn min_point() -> Point {
        Point(BaseVector::from_value(f64::MIN))
    }

    pub fn base(&self) -> &BaseVector { &self.0 }

    pub fn max(&self, other: Point) -> Point {
        Point::new(self.base().x.max(other.base().x), self.base().y.max(other.base().y),self.base().z.max(other.base().z))
    }

    pub fn min(&self, other: Point) -> Point {
        Point::new(self.base().x.min(other.base().x), self.base().y.min(other.base().y),self.base().z.min(other.base().z))
    }
}

impl From<BaseVector> for Point {
    fn from(base: BaseVector) -> Self {
        Point( base )
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(mut self, rhs: Vector) -> Point {
        self.0 = self.0 + rhs.0;
        self
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        let base = self.0 - rhs.0;
        Vector::from(base)
    }
}

impl Sub<Vector> for Point {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let base = self.0 - rhs.0;
        Vector::from(base)
    }
}

impl Mul<f64> for Point{
    type Output = Point;

    fn mul(mut self, rhs: f64) -> Point {
        self.0 = self.0*rhs;
        self
    }
}

impl Mul<Point> for f64{
    type Output = Point;

    fn mul(self, mut rhs: Point) -> Point {
        rhs.0 = rhs.0*self;
        rhs
    }
}

//////////////////
//Vector
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(BaseVector);
pub type Direction = Vector;
pub type Normal = Vector;

impl Vector {
    pub fn from_base(base: BaseVector) -> Vector
    {
        Vector( base )
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vector
    {
        Vector( BaseVector::new( x, y, z ) )
    }
    pub fn up() -> Vector { Vector( BaseVector::new( 0.0, 1.0, 0.0 ) ) }

    pub fn base(&self) -> &BaseVector { &self.0 }

    pub fn length(&self) -> f64 {
        (self.0*self.0).sum().sqrt()
    }

    pub fn invert(mut self) -> Vector {
        self.0.x = -self.0.x;
        self.0.y = -self.0.y;
        self.0.z = -self.0.z;
        self
    }

    pub fn normalize(mut self) -> Vector{
        self.0 = self.0 / self.length();
        self
    }

    pub fn cross(self, other: Vector) -> Vector {
        let x = self.base().y*other.base().z - self.base().z*other.base().y;
        let y = self.base().z*other.base().x - self.base().x*other.base().z;
        let z = self.base().x*other.base().y - self.base().y*other.base().x;
        Vector(BaseVector{x,y,z})
    }
}

impl From<BaseVector> for Vector {
    fn from(base: BaseVector) -> Self {
        Vector( base )
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(mut self, rhs: Vector) -> Vector {
        self.0 = rhs.0 + self.0;
        self
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, mut rhs: Point) -> Point {
        rhs.0 = rhs.0 + self.0;
        rhs
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(mut self, rhs: Vector) -> Vector {
        self.0 = self.0 - rhs.0;
        self
    }
}

impl Mul<f64> for Vector{
    type Output = Vector;

    fn mul(mut self, rhs: f64) -> Vector {
        self.0 = self.0*rhs;
        self
    }
}

impl Mul<Vector> for f64{
    type Output = Vector;

    fn mul(self, mut rhs: Vector) -> Vector {
        rhs.0 = rhs.0*self;
        rhs
    }
}