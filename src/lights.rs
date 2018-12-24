
use std::f64::consts::PI;

use math::{VectorTrait, Normal, Direction, Point, Radiance};
use primitives::{Object, Rectangle};
use material::Color;
use scene::{SceneGraph, Intersection};
use sampling;

pub trait Light : Send + Sync{
    fn light_points(&self) -> Vec<(Point,Option<Normal>)>;
    fn radiance_from_point(&self, Point) -> Radiance;
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
    fn light_points(&self) -> Vec<(Point, Option<Normal>)> {
        vec![(self.position, None)]
    }

    fn radiance_from_point(&self, _: Point) -> Radiance {
        let factor = self.power / (4.0*PI);
        let rad = Radiance::gray(factor);
        rad.apply_color(self.color)
    }
}

pub struct SurfaceLight {
    surface: Rectangle,
    power: f64,
    color: Color
}

impl SurfaceLight {
    pub fn new(surface: Rectangle, power: f64, color: Color) -> SurfaceLight{
        SurfaceLight{surface, power, color}
    }
}

impl Light for SurfaceLight {
    fn light_points(&self) -> Vec<(Point, Option<Normal>)> {
        let normal: Option<Normal> = Some( (self.surface.transformation().inverted().transpose() * Normal::new(0.0, 1.0, 0.0)).normalize() );
        let amt_samples = 64 as usize;

        let mut vec = Vec::new();
        for _ in 0..amt_samples {
            let (u,v) = sampling::sample_rect(1.0,1.0);
            let point = *self.surface.transformation().matrix()*(Point::new(self.surface.corner_point().x()*u, 0.0, self.surface.corner_point().z()*v));
            vec.push((point,normal));
        }
        vec
    }

    fn radiance_from_point(&self, _: Point) -> Radiance {
        let factor = self.power / (2.0*PI);
        let rad = Radiance::gray(factor);
        rad.apply_color(self.color)
    }

}