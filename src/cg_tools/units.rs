
use std::ops::{Add, Mul, Div};
use super::{Color,ColorModel};
use crate::settings;

//////////////////
//Radiance
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Radiance(Color);

impl Radiance {
    pub fn zero() -> Radiance{
        Radiance(Color::black())
    }
    pub fn new(r: f64,g: f64,b: f64) -> Radiance { Radiance(Color::new_rgb(r,g,b)) }
    pub fn gray_scale(rad:f64) -> Radiance { Radiance(Color::gray_scale(rad)) }

    pub fn color(&self) -> Color { self.0 }
}

impl From<Color> for Radiance {
    fn from(color: Color) -> Self {
        let mut color = color;
        match settings::get().color_model {
            ColorModel::RGB => color.convert_to_rgb(),
            ColorModel::XYZ => color.convert_to_xyz(),
        };
        Radiance(color)
    }
}

impl From<Radiance> for Color {
    fn from(rad: Radiance) -> Self {
        rad.0
    }
}

impl Add<Radiance> for Radiance{
    type Output = Radiance;

    fn add(self, rhs: Radiance) -> Radiance {
        Radiance(self.0+rhs.0)
    }
}

impl Mul<Radiance> for Radiance{
    type Output = Radiance;

    fn mul(self, rhs: Radiance) -> Radiance {
        Radiance(self.0*rhs.0)
    }
}

impl Mul<f64> for Radiance{
    type Output = Radiance;

    fn mul(self, rhs: f64) -> Radiance {
        Radiance(self.0*rhs)
    }
}

impl Mul<Radiance> for f64{
    type Output = Radiance;

    fn mul(self, rhs: Radiance) -> Radiance {
        Radiance(self*rhs.0)
    }
}

impl Div<f64> for Radiance{
    type Output = Radiance;

    fn div(self, rhs: f64) -> Radiance {
        Radiance(self.0/rhs)
    }
}

impl Div<Radiance> for f64{
    type Output = Radiance;

    fn div(self, rhs: Radiance) -> Radiance {
        Radiance(self/rhs.0)
    }
}