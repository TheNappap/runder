
use std::ops::{Add,Sub,Mul};
use super::base::BaseVector;
use super::utils::VectorTrait;

//////////////////
//Point
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(BaseVector);

impl Point {
    pub fn origin() -> Point
    {
        Point( BaseVector { x:0.0, y:0.0, z:0.0 } )
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point
    {
        Point( BaseVector { x, y, z } )
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
        let x = self.0.x - rhs.0.x;
        let y = self.0.y - rhs.0.y;
        let z = self.0.z - rhs.0.z;
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
        Vector( BaseVector { x, y, z } )
    }

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

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, mut rhs: Point) -> Point {
        rhs.0 = rhs.0 + self.0;
        rhs
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