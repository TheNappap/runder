
extern crate itertools;
use self::itertools::iproduct;

use std::sync::{Arc};
use std::f64::consts::{PI};

use settings::Settings;
use math::{Point, Direction};
use cg_tools::Ray;

#[derive(Clone, Debug)]
pub struct Pixel{
    pub x : i32,
    pub y : i32,
}

#[derive(Clone)]
pub struct PerspectiveCamera {
    settings: Arc<Settings>,
    position: Point,
    direction: Direction,
    up: Direction,
    right: Direction,
    fov: f64
}

impl PerspectiveCamera {

    pub fn new(settings: Arc<Settings>, position: Point, direction: Direction, up: Direction, fov: f64) -> PerspectiveCamera{
        let right = direction.cross(&up);
        let up = right.cross(&direction);
        PerspectiveCamera{settings, position, direction: direction, up, right, fov: fov*(PI/180.0) }
    }

    pub fn position(&self) -> Point { self.position }
    pub fn direction(&self) -> Direction { self.direction }
    pub fn up(&self) -> Direction { self.up }
    pub fn fov(&self) -> f64 { self.fov }

    pub fn rays_for_pixel(&self, pixel: &Pixel) -> Vec<Ray>
    {
        let multi_sample = self.settings.aa_multi_sample as f64;
        let width = self.settings.screen_width as f64;
        let height = self.settings.screen_height as f64;
        let sample_width = (self.fov/2.0).tan() * 2.0/width / multi_sample;
        let sample_height = sample_width;

        let dir = *self.direction - ((width/2.0 - pixel.x as f64)*multi_sample - 0.5)*sample_width**self.right + ((height/2.0 - pixel.y as f64)*multi_sample - 0.5)*sample_height**self.up;
        iproduct!(0..self.settings.aa_multi_sample,0..self.settings.aa_multi_sample).map(|(i,j)| {
            let direction = Direction::from(dir + (j as f64)*sample_width**self.right - (i as f64)*sample_height**self.up );
            Ray::new(self.position, direction)
        }).collect()
    }
}