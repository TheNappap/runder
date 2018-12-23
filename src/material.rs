
use std::f64::consts::PI;
use std::fmt::Debug;

use math::{Radiance,Direction};

//////////////////
//Color
//////////////////
#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn black() -> Color {
        Color{r:0.0,g:0.0,b:0.0}
    }

    pub fn gray(value: f64) -> Color{
        Color{r:value,g:value,b:value}
    }

    pub fn new(r: f64, g: f64, b: f64) -> Color{
        Color{r,g,b}
    }

    pub fn r(&self) -> f64 { self.r }
    pub fn g(&self) -> f64 { self.g }
    pub fn b(&self) -> f64 { self.b }

    pub fn radiance_to_color(rad: Radiance) -> Color{
        let r = rad.r().min(1.0);
        let g =  rad.g().min(1.0);
        let b =  rad.b().min(1.0);
        Color{r, g, b}
    }
}

//////////////////
//Material
//////////////////
pub trait Material : Send + Sync + Debug{
    fn brdf(&self, incoming: Direction, outgoing: Direction) -> Color;
}

//////////////////
//Lambertian
//////////////////
#[derive(Debug)]
pub struct Lambertian {
    color: Color
}

impl Lambertian{
    pub fn new(color: Color) -> Lambertian {
        Lambertian{color}
    }
}

impl Material for Lambertian {
    fn brdf(&self, _: Direction, _: Direction) -> Color {
        let factor = 1.0/(2.0*PI);
        Color::new(self.color.r*factor, self.color.g*factor, self.color.b*factor)
    }
}