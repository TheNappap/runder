
use std::ops::{Add, Mul};
use math::{Matrix,Vector};
use settings::{self,ColorModel};

#[derive(Copy, Clone)]
pub enum WhiteReference {
    A,B,C,
    D50,D55,D65,D75,
    E,
    F2,F7,F11
}

//////////////////
//RGB and XYZ conversions
//////////////////
// Source: http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
// CIE RGB (E)
const CIE_RGB_TO_XYZ: [[f64; 3]; 3] = [ [0.4887180,  0.3106803,  0.2006017],
                                        [0.1762044,  0.8129847,  0.0108109],
                                        [0.0000000,  0.0102048,  0.9897952] ];
const XYZ_TO_CIE_RGB: [[f64; 3]; 3] = [ [2.3706743, -0.9000405, -0.4706338],
                                        [-0.5138850,  1.4253036,  0.0885814],
                                        [0.0052982, -0.0146949,  1.0093968] ];


//////////////////
//Color
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    RGB{r:f64,g:f64,b:f64},
    XYZ{x:f64,y:f64,z:f64}
}

impl Color {
    pub fn black() -> Color {
        Color::RGB{r:0.,g:0.,b:0.}
    }

    pub fn white(white_ref: WhiteReference) -> Color {
        match white_ref {
            WhiteReference::A => Color::XYZ{x:1.09850,y:1.,z:0.35585},
            WhiteReference::B => Color::XYZ{x:0.99072,y:1.,z:0.85223},
            WhiteReference::C => Color::XYZ{x:0.98074,y:1.,z:1.18232},
            WhiteReference::D50 => Color::XYZ{x:0.96422,y:1.,z:0.82521},
            WhiteReference::D55 => Color::XYZ{x:0.95682,y:1.,z:0.92149},
            WhiteReference::D65 => Color::XYZ{x:0.95047,y:1.,z:1.08883},
            WhiteReference::D75 => Color::XYZ{x:0.94972,y:1.,z:1.22638},
            WhiteReference::E => Color::XYZ{x:1.,y:1.,z:1.},
            WhiteReference::F2 => Color::XYZ{x:0.99186,y:1.,z:0.67393},
            WhiteReference::F7 => Color::XYZ{x:0.95041,y:1.,z:1.08747},
            WhiteReference::F11 => Color::XYZ{x:1.00962,y:1.,z:0.64350},
        }
    }

    pub fn gray_scale(value: f64) -> Color{
        Color::RGB{r:value,g:value,b:value}
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Color{
        Color::RGB{r,g,b}
    }

    pub fn new_xyz(x: f64, y: f64, z: f64) -> Color{
        Color::XYZ{x,y,z}
    }

    pub fn rgb(self) -> (f64,f64,f64) {
        match self {
            Color::RGB {r,g,b} => (r,g,b),
            Color::XYZ {x,y,z} => {
                let rgb = Matrix::from(XYZ_TO_CIE_RGB)*Vector::new(x,y,z);
                rgb.as_tuple()
            }
        }
    }

    pub fn xyz(self) -> (f64,f64,f64) {
        match self {
            Color::XYZ {x,y,z} => (x,y,z),
            Color::RGB {r,g,b} => {
                let xyz = Matrix::from(CIE_RGB_TO_XYZ)*Vector::new(r,g,b);
                xyz.as_tuple()
            }
        }
    }

    pub fn convert_to_rgb(&mut self) {
        let (r,g,b) = self.rgb();
        *self = Color::new_rgb(r,g,b);
    }

    pub fn convert_to_xyz(&mut self) {
        let (x,y,z) = self.xyz();
        *self = Color::new_xyz(x,y,z);
    }

    pub fn gamma_correct(self, gamma: f64) -> Color {
        let inv_gamma = 1./gamma;
        let (r,g,b) = self.rgb();
        let r = r.powf(inv_gamma);
        let g = g.powf(inv_gamma);
        let b = b.powf(inv_gamma);
        Color::RGB{r,g,b}
    }

    pub fn clamped_rgb(self) -> Color {
        let (r,g,b) = self.rgb();
        let r = r.max(0.).min(1.);
        let g = g.max(0.).min(1.);
        let b = b.max(0.).min(1.);
        Color::RGB{r,g,b}
    }
}

impl Add<Color> for Color{
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        match settings::get().color_model {
            ColorModel::RGB => {
                let rgb1 = self.rgb();
                let rgb2 = rhs.rgb();
                Color::RGB {r:rgb1.0+rgb2.0,g:rgb1.1+rgb2.1,b:rgb1.2+rgb2.2}
            },
            ColorModel::XYZ => {
                let xyz1 = self.xyz();
                let xyz2 = rhs.xyz();
                Color::XYZ {x:xyz1.0+xyz2.0,y:xyz1.1+xyz2.1,z:xyz1.2+xyz2.2}
            },
        }
    }
}

impl Mul<Color> for Color{
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        match settings::get().color_model {
            ColorModel::RGB => {
                let rgb1 = self.rgb();
                let rgb2 = rhs.rgb();
                Color::RGB {r:rgb1.0*rgb2.0,g:rgb1.1*rgb2.1,b:rgb1.2*rgb2.2}
            },
            ColorModel::XYZ => {
                let xyz1 = self.xyz();
                let xyz2 = rhs.xyz();
                Color::XYZ {x:xyz1.0*xyz2.0,y:xyz1.1*xyz2.1,z:xyz1.2*xyz2.2}
            },
        }
    }
}

impl Mul<f64> for Color{
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        match settings::get().color_model {
            ColorModel::RGB => {
                let rgb = self.rgb();
                Color::RGB {r:rgb.0*rhs,g:rgb.1*rhs,b:rgb.2*rhs}
            },
            ColorModel::XYZ => {
                let xyz = self.xyz();
                Color::XYZ {x:xyz.0*rhs,y:xyz.1*rhs,z:xyz.2*rhs}
            },
        }
    }
}

impl Mul<Color> for f64{
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        match settings::get().color_model {
            ColorModel::RGB => {
                let rgb = rhs.rgb();
                Color::RGB {r:rgb.0*self,g:rgb.1*self,b:rgb.2*self}
            },
            ColorModel::XYZ => {
                let xyz = rhs.xyz();
                Color::XYZ {x:xyz.0*self,y:xyz.1*self,z:xyz.2*self}
            },
        }
    }
}
