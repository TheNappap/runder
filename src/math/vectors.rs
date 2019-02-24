
use std::ops::{Add,Sub,Mul,Deref};
use std::f64;

use super::base_vector::BaseVector;

//////////////////
//Point
//////////////////
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Point(BaseVector);

impl Point {
    pub fn origin() -> Point { Point( BaseVector::new(0.0, 0.0,0.0 ) ) }
    pub fn new(x: f64, y: f64, z: f64) -> Point
    {
        Point( BaseVector::new( x, y, z ) )
    }
    pub fn max_point() -> Point { Point(BaseVector::from_value(f64::MAX)) }
    pub fn min_point() -> Point {
        Point(BaseVector::from_value(f64::MIN))
    }

    pub fn max(&self, other: Point) -> Point {
        Point::new(self.x.max(other.x), self.y.max(other.y),self.z.max(other.z))
    }

    pub fn min(&self, other: Point) -> Point {
        Point::new(self.x.min(other.x), self.y.min(other.y),self.z.min(other.z))
    }
}

impl From<BaseVector> for Point {
    fn from(base: BaseVector) -> Self {
        Point( base )
    }
}

impl Deref for Point {
    type Target = BaseVector;

    fn deref(&self) -> &BaseVector {
       &self.0
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
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector(BaseVector);

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
        self.0.x = -self.0.x;
        self.0.y = -self.0.y;
        self.0.z = -self.0.z;
        self
    }

    pub fn normalize(mut self) -> Vector{
        self.0 = self.0 / self.length();
        self
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        let x = self.y*other.z - self.z*other.y;
        let y = self.z*other.x - self.x*other.z;
        let z = self.x*other.y - self.y*other.x;
        Vector(BaseVector{x,y,z})
    }
}

impl From<BaseVector> for Vector {
    fn from(base: BaseVector) -> Self {
        Vector( base )
    }
}

impl Deref for Vector {
    type Target = BaseVector;

    fn deref(&self) -> &BaseVector {
        &self.0
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

//////////////////
//Direction
//////////////////
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Direction(Vector);

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Direction { Direction( Vector::new( x, y, z ).normalize() ) }
    pub fn up() -> Direction { Direction( Vector::new( 0.0, 1.0, 0.0 ) ) }
    pub fn posz() -> Direction { Direction( Vector::new( 0.0, 0.0, 1.0 ) ) }
    pub fn negz() -> Direction { Direction( Vector::new( 0.0, 0.0, -1.0 ) ) }

    pub fn invert(self) -> Direction {
        Direction(self.0.invert())
    }

    pub fn cross(&self, other: &Vector) -> Direction {
        Direction::from(self.0.cross(other))
    }
}

impl From<Vector> for Direction {
    fn from(vector: Vector) -> Self {
        Direction( vector.normalize() )
    }
}

impl Deref for Direction {
    type Target = Vector;

    fn deref(&self) -> &Vector {
        &self.0
    }
}

//////////////////
//Normal
//////////////////
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Normal(Vector);

impl Normal {
    pub fn new(x: f64, y: f64, z: f64) -> Normal { Normal( Vector::new( x, y, z ).normalize() ) }
    pub fn up() -> Normal { Normal( Vector::new( 0.0, 1.0, 0.0 ) ) }

    pub fn invert(self) -> Normal {
        Normal(self.0.invert())
    }

    pub fn cross(&self, other: &Vector) -> Normal {
        Normal::from(self.0.cross(other))
    }
}

impl From<Vector> for Normal {
    fn from(vector: Vector) -> Self {
        Normal( vector.normalize() )
    }
}

impl Deref for Normal {
    type Target = Vector;

    fn deref(&self) -> &Vector {
        &self.0
    }
}