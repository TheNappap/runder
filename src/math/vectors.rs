
use std::ops::{Add,Sub,Mul};
use std::f64;

use super::base_vector::BaseVector;
use units::Color;

//////////////////
//VectorTrait
//////////////////
pub trait VectorTrait{
    fn base(&self) -> &BaseVector;
    fn x(&self) -> f64 { return self.base().x() }
    fn y(&self) -> f64 { return self.base().y() }
    fn z(&self) -> f64 { return self.base().z() }

    fn sum(&self) -> f64 { self.base().x()+self.base().y()+self.base().z() }
    fn dot(&self, v : &VectorTrait) -> f64 { (*self.base()**v.base()).sum()}
}

//////////////////
//Point
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(BaseVector);

impl Point {
    pub fn origin() -> Point
    {
        Point( BaseVector::new(0.0, 0.0,0.0 ) )
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point
    {
        Point( BaseVector::new( x, y, z ) )
    }

    pub fn max_point() -> Point {
        Point(BaseVector::new_same_element(f64::MAX))
    }

    pub fn min_point() -> Point {
        Point(BaseVector::new_same_element(f64::MIN))
    }

    pub fn max(&self, other: Point) -> Point {
        Point::new(self.x().max(other.x()), self.y().max(other.y()),self.z().max(other.z()))
    }

    pub fn min(&self, other: Point) -> Point {
        Point::new(self.x().min(other.x()), self.y().min(other.y()),self.z().min(other.z()))
    }
}

impl VectorTrait for Point{
    fn base(&self) -> &BaseVector { &self.0 }
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
        let x = self.0.x() - rhs.0.x();
        let y = self.0.y() - rhs.0.y();
        let z = self.0.z() - rhs.0.z();
        Vector::new(x,y,z)
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
    pub fn new(x: f64, y: f64, z: f64) -> Vector
    {
        Vector( BaseVector::new( x, y, z ) )
    }

    pub fn up() -> Vector { Vector( BaseVector::new( 0.0, 1.0, 0.0 ) ) }

    pub fn length(&self) -> f64 {
        (self.0*self.0).sum().sqrt()
    }

    pub fn invert(mut self) -> Vector {
        self.0 = self.0.invert();
        self
    }

    pub fn normalize(mut self) -> Vector{
        self.0 = self.0 / self.length();
        self
    }

    pub fn cross(self, other: Vector) -> Vector {
        let x = self.y()*other.z() - self.z()*other.y();
        let y = self.z()*other.x() - self.x()*other.z();
        let z = self.x()*other.y() - self.y()*other.x();
        Vector::new(x,y,z)
    }
}

impl VectorTrait for Vector{
    fn base(&self) -> &BaseVector { &self.0 }
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