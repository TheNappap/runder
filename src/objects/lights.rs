
use std::f64::consts::PI;

use crate::math::{Normal, Point};
use super::{Object, Rectangle};
use crate::cg_tools::{Color, Radiance, SamplingTechnique, Transformation};

pub trait Light : Send + Sync{
    fn light_points(&self, sampling_technique: SamplingTechnique) -> Vec<(Point,Option<Normal>)>;
    fn radiance_from_point(&self, point: Point) -> Radiance;
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
        let rad = Radiance::gray_scale(factor);
        rad*Radiance::from(self.color)
    }
}

pub struct SurfaceLight {
    surface: Rectangle,
    transformation: Transformation,
    power: f64,
    color: Color
}

impl SurfaceLight {
    pub fn new(surface: Rectangle, power: f64, color: Color) -> SurfaceLight{
        SurfaceLight::transformed(surface,Transformation::new(), power, color)
    }

    pub fn transformed(surface: Rectangle, transformation: Transformation, power: f64, color: Color) -> SurfaceLight{
        SurfaceLight{surface, transformation, power, color}
    }
}

impl Light for SurfaceLight {
    fn light_points(&self, technique: SamplingTechnique) -> Vec<(Point, Option<Normal>)> {
        let normal: Option<Normal> = Some( self.transformation.inverted().transpose() * Normal::new(0.0, 1.0, 0.0) );

        technique.sample_rect(1.0,1.0).iter().map(|(u, v) |{
            let points = self.surface.points();
            let u_vector = points[1] - points[0];
            let v_vector = points[3] - points[0];

            let vector = *u*u_vector + *v*v_vector;
            let point = self.transformation.matrix()*(points[0]+vector);
            (point, normal)
        }).collect()
    }

    fn radiance_from_point(&self, _: Point) -> Radiance {
        let factor = self.power / (2.0*PI);
        let rad = Radiance::gray_scale(factor);
        rad*Radiance::from(self.color)
    }

}