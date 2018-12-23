
use std::f64::consts::PI;

use math::{VectorTrait, Normal, Direction, Point, Radiance};
use material::Color;

pub trait Light : Send + Sync{
    fn receive_radiance(&self, point: &Point, normal : &Normal) -> (Radiance, Direction);
}

pub struct PointLight {
    position: Point,
    power: f64,
    color: Color
}

impl PointLight {
    pub fn new(position: Point, power: f64, color: Color) -> PointLight{
        PointLight{position, power, color}
    }
}

impl Light for PointLight {
    fn receive_radiance(&self, receiving_point: &Point, normal : &Normal) -> (Radiance, Direction) {
        let dir = self.position - *receiving_point;
        let r = dir.length();

        let factor = self.power/(r*r*4.0*PI);
        (Radiance::new(self.color.r()*factor, self.color.g()*factor, self.color.b()*factor), dir)
    }
}