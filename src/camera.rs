
use math::{Point, Direction, Ray};

pub struct Pixel{
    pub x : i32,
    pub y : i32,
}

#[derive(Copy, Clone)]
pub struct PerspectiveCamera {
    position: Point,
    direction: Direction,
    up: Direction,
    fov: f64
}

impl PerspectiveCamera {

    pub fn new(position: Point, direction: Direction, up: Direction, fov: f64) -> PerspectiveCamera{
        PerspectiveCamera{position,direction,up,fov}
    }

    pub fn position(&self) -> Point { self.position }
    pub fn direction(&self) -> Direction { self.direction }
    pub fn up(&self) -> Direction { self.up }
    pub fn fov(&self) -> f64 { self.fov }

    pub fn ray_for_pixel(&self, width: i32, height: i32, pixel: Pixel) -> Ray
    {
        let width = width as f64;
        let height = height as f64;
        let x = (pixel.x as f64 - width/2.0) as f64;
        let y = (-pixel.y as f64 + height/2.0) as f64;
        let z = -(height/2.0)/(self.fov/2.0).tan();
        let direction = Direction::new(x, y, z);
        Ray::new(self.position, direction)
    }
}