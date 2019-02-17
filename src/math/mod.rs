mod base_vector;
mod vectors;
mod matrix;

pub use self::vectors::{Point, Vector, Direction, Normal};
pub use self::matrix::{Matrix, RotationAxis};

pub const EPSILON: f64 = 1e-12;