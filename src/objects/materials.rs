
use std::f64::consts::PI;
use std::fmt::Debug;

use math::Direction;
use units::Color;

//////////////////
//Material
//////////////////
pub trait Material : Send + Sync + Debug{
    fn brdf(&self, incoming: Direction, outgoing: Direction) -> Color;
}

//////////////////
//Lambertian
//////////////////
#[derive(Clone, Debug)]
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
        Color::new(self.color.r()*factor, self.color.g()*factor, self.color.b()*factor)
    }
}