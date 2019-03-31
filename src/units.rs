
use std::ops::{Add, Mul};

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


//////////////////
//Color
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn gamma_correct(&mut self, gamma: f64) {
        let inv_gamma = 1./gamma;
        self.r = self.r.powf(inv_gamma);
        self.g = self.g.powf(inv_gamma);
        self.b = self.b.powf(inv_gamma);
    }
}