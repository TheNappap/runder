
use std::ops::{Add,Mul};
use super::types::*;
use super::base::BaseVector;
use super::super::material::Color;

//////////////////
//VectorTrait
//////////////////
pub trait VectorTrait{
    fn base(&self) -> &BaseVector;
    fn x(&self) -> f64 { return self.base().x }
    fn y(&self) -> f64 { return self.base().y }
    fn z(&self) -> f64 { return self.base().z }

    fn sum(&self) -> f64 { self.base().x+self.base().y+self.base().z }
    fn dot(&self, v : &VectorTrait) -> f64 { (*self.base()**v.base()).sum()}
}

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
//Radiance
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Radiance{
    r:f64,
    g:f64,
    b:f64
}

impl Radiance {
    pub fn zero() -> Radiance{
        Radiance{r:0.0,g:0.0,b:0.0}
    }

    pub fn gray(rad:f64) -> Radiance{
        Radiance{r:rad,g:rad,b:rad}
    }

    pub fn new(r:f64, g:f64, b:f64) -> Radiance{
        Radiance{r,g,b}
    }

    pub fn r(&self) -> f64 { self.r }
    pub fn g(&self) -> f64 { self.g }
    pub fn b(&self) -> f64 { self.b }

    pub fn apply_color(mut self, color: Color) -> Radiance{
        self.r = self.r*color.r();
        self.g = self.g*color.g();
        self.b = self.b*color.b();
        self
    }
}

impl Add<Radiance> for Radiance{
    type Output = Radiance;

    fn add(self, mut rhs: Radiance) -> Radiance {
        rhs.r = rhs.r+self.r;
        rhs.g = rhs.g+self.g;
        rhs.b = rhs.b+self.b;
        rhs
    }
}

impl Mul<f64> for Radiance{
    type Output = Radiance;

    fn mul(mut self, rhs: f64) -> Radiance {
        self.r = self.r*rhs;
        self.g = self.g*rhs;
        self.b = self.b*rhs;
        self
    }
}

impl Mul<Radiance> for f64{
    type Output = Radiance;

    fn mul(self, mut rhs: Radiance) -> Radiance {
        rhs.r = rhs.r*self;
        rhs.g = rhs.g*self;
        rhs.b = rhs.b*self;
        rhs
    }
}