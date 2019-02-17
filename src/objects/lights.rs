
use std::f64::consts::PI;

use math::{Normal, Point};
use super::{Object, Rectangle};
use units::{Color, Radiance};
use cg_tools::SamplingTechnique;

pub trait Light : Send + Sync{
    fn light_points(&self, sampling_technique: SamplingTechnique) -> Vec<(Point,Option<Normal>)>;
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
    fn light_points(&self, _:SamplingTechnique) -> Vec<(Point, Option<Normal>)> {
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
    fn light_points(&self, technique: SamplingTechnique) -> Vec<(Point, Option<Normal>)> {
        let normal: Option<Normal> = Some( (&self.surface.transformation().inverted().transpose() * Normal::new(0.0, 1.0, 0.0)) );

        technique.sample_rect(1.0,1.0).iter().map(|(u, v) |{
            let points = self.surface.points();
            let u_vector = points[1] - points[0];
            let v_vector = points[3] - points[0];

            let vector = *u*u_vector + *v*v_vector;
            let point = self.surface.transformation().matrix()*(points[0]+vector);
            (point, normal)
        }).collect()
    }

    fn radiance_from_point(&self, _: Point) -> Radiance {
        let factor = self.power / (2.0*PI);
        let rad = Radiance::gray(factor);
        rad.apply_color(self.color)
    }

}