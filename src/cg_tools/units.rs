
use std::ops::{Add, Mul};
use super::Color;

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
}

impl From<Color> for Radiance {
    fn from(color: Color) -> Self {
        Radiance{r:color.r(),g:color.g(),b:color.b()}
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

impl Mul<Radiance> for Radiance{
    type Output = Radiance;

    fn mul(mut self, rhs: Radiance) -> Radiance {
        self.r = self.r*rhs.r;
        self.g = self.g*rhs.g;
        self.b = self.b*rhs.b;
        self
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