
use super::Radiance;

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

    pub fn gamma_correct(&mut self, gamma: f64) {
        let inv_gamma = 1./gamma;
        self.r = self.r.powf(inv_gamma);
        self.g = self.g.powf(inv_gamma);
        self.b = self.b.powf(inv_gamma);
    }
}

impl From<Radiance> for Color {
    fn from(rad: Radiance) -> Self {
        Color{r:rad.r().min(1.0),g:rad.g().min(1.0),b:rad.b().min(1.0)}
    }
}
