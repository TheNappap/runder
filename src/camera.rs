
extern crate itertools;
use self::itertools::iproduct;

use std::sync::{Arc};
use std::f64::consts::PI;
use settings::Settings;
use math::{Point, Direction, Ray};

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
    fov: f64
}

impl PerspectiveCamera {

    pub fn new(settings: Arc<Settings>, position: Point, direction: Direction, up: Direction, fov: f64) -> PerspectiveCamera{
        PerspectiveCamera{settings, position,direction,up,fov}
    }

    pub fn position(&self) -> Point { self.position }
    pub fn direction(&self) -> Direction { self.direction }
    pub fn up(&self) -> Direction { self.up }
    pub fn fov(&self) -> f64 { self.fov }

    pub fn rays_for_pixel(&self, pixel: &Pixel) -> Vec<Ray>
    {
        let multi_sample = self.settings.aa_multi_sample;
        let width = self.settings.screen_width as f64;
        let height = self.settings.screen_height as f64;
        iproduct!(0..multi_sample,0..multi_sample).map(|(i,j)| {
            let ratio = width / height;
            let x = (2.0 * (( (multi_sample*pixel.x + i) as f64 + 0.5) / (width*multi_sample as f64) ) - 1.0) * (self.fov / 2.0 * PI / 180.0).tan() * ratio;
            let y = (1.0 - 2.0 * (( (multi_sample*pixel.y + j) as f64 + 0.5) / (height*multi_sample as f64) ) * (self.fov / 2.0 * PI / 180.0).tan());
            let direction = Direction::new(x, y, -1.0);
            Ray::new(self.position, direction)
        }).collect()
    }
}