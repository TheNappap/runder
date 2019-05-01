
use std::ops::{Add, Mul};
use math::{Matrix,Vector};

//////////////////
//Color
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    RGB{r:f64,g:f64,b:f64},
    XYZ{x:f64,y:f64,z:f64}
}

// Source: http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
// NTSC RGB (C)
/*const RGB_TO_XYZ: [[f64; 3]; 3] = [ [0.6068909,  0.1735011,  0.2003480],
                                    [0.2989164,  0.5865990,  0.1144845],
                                    [0.0000000,  0.0660957,  1.1162243] ];
const XYZ_TO_RGB: [[f64; 3]; 3] = [ [1.9099961, -0.5324542, -0.2882091],
                                    [-0.9846663,  1.9991710, -0.0283082],
                                    [0.0583056, -0.1183781,  0.8975535] ];*/
// Best RGB (D50)
/*const RGB_TO_XYZ: [[f64; 3]; 3] = [ [0.6326696,  0.2045558,  0.1269946],
                                    [0.2284569,  0.7373523,  0.0341908],
                                    [0.0000000,  0.0095142,  0.8156958] ];
const XYZ_TO_RGB: [[f64; 3]; 3] = [ [1.7552599, -0.4836786, -0.2530000],
                                    [-0.5441336,  1.5068789,  0.0215528],
                                    [0.0063467, -0.0175761,  1.2256959] ];*/
// sRGB (D65)
/*const RGB_TO_XYZ: [[f64; 3]; 3] = [ [0.4124564,  0.3575761,  0.1804375],
                                    [0.2126729,  0.7151522,  0.0721750],
                                    [0.0193339,  0.1191920,  0.9503041] ];
const XYZ_TO_RGB: [[f64; 3]; 3] = [ [3.2404542, -1.5371385, -0.4985314],
                                    [-0.9692660,  1.8760108,  0.0415560],
                                    [0.0556434, -0.2040259,  1.0572252] ];*/
// CIE RGB (E)
const RGB_TO_XYZ: [[f64; 3]; 3] = [ [0.4887180,  0.3106803,  0.2006017],
                                    [0.1762044,  0.8129847,  0.0108109],
                                    [0.0000000,  0.0102048,  0.9897952] ];
const XYZ_TO_RGB: [[f64; 3]; 3] = [ [2.3706743, -0.9000405, -0.4706338],
                                    [-0.5138850,  1.4253036,  0.0885814],
                                    [0.0052982, -0.0146949,  1.0093968] ];

impl Color {
    pub fn black() -> Color {
        Color::RGB{r:0.,g:0.,b:0.}
    }

    pub fn gray_scale(value: f64) -> Color{
        Color::RGB{r:value,g:value,b:value}
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Color{
        Color::RGB{r,g,b}
    }

    pub fn new_xyz(r: f64, g: f64, b: f64) -> Color{
        Color::RGB{r,g,b}
    }

    pub fn rgb(self) -> (f64,f64,f64) {
        match self {
            Color::RGB {r,g,b} => (r,g,b),
            Color::XYZ {x,y,z} => {
                let rgb = &Matrix::from(XYZ_TO_RGB)*Vector::new(x, y, z);
                rgb.as_tuple()
            }
        }
    }

    pub fn xyz(self) -> (f64,f64,f64) {
        match self {
            Color::XYZ {x,y,z} => (x,y,z),
            Color::RGB {r,g,b} => {
                let xyz = &Matrix::from(RGB_TO_XYZ)*Vector::new(r, g, b);
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
        let r = r.min(1.);
        let g = g.min(1.);
        let b = b.min(1.);
        Color::RGB{r,g,b}
    }
}

impl Add<Color> for Color{
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        let rgb1 = self.rgb();
        let rgb2 = rhs.rgb();
        Color::RGB {r:rgb1.0+rgb2.0,g:rgb1.1+rgb2.1,b:rgb1.2+rgb2.2}
    }
}

impl Mul<Color> for Color{
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        let rgb1 = self.rgb();
        let rgb2 = rhs.rgb();
        Color::RGB {r:rgb1.0*rgb2.0,g:rgb1.1*rgb2.1,b:rgb1.2*rgb2.2}
    }
}

impl Mul<f64> for Color{
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        let rgb = self.rgb();
        Color::RGB {r:rgb.0*rhs,g:rgb.1*rhs,b:rgb.2*rhs}
    }
}

impl Mul<Color> for f64{
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        let rgb = rhs.rgb();
        Color::RGB {r:rgb.0*self,g:rgb.1*self,b:rgb.2*self}
    }
}
