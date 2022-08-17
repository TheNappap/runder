
use crate::math::{Matrix, RotationAxis, Vector};

#[derive(Clone, Debug)]
pub struct Transformation {
    matrix: Matrix,
    inverted: Matrix
}

impl Transformation {
    pub fn new() -> Transformation {
        let mat = Matrix::identiy();
        Transformation{matrix: mat.clone(), inverted: mat}
    }

    pub fn matrix(&self) -> Matrix{ self.matrix }
    pub fn inverted(&self) -> Matrix{ self.inverted }

    pub fn translate(mut self, vector: Vector) -> Transformation {
        self.matrix = Matrix::translated(vector) * self.matrix;
        self.inverted = self.inverted * Matrix::translated(vector.invert());
        self
    }

    pub fn scale_all(self, scale: f64) -> Transformation {
        self.scale(scale,scale,scale)
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Transformation {
        self.matrix = Matrix::scaled(x,y,z) * self.matrix;
        self.inverted = self.inverted * Matrix::scaled(1.0/x,1.0/y,1.0/z);
        self
    }

    pub fn rotate(mut self, axis: RotationAxis, radians: f64) -> Transformation{
        self.matrix = Matrix::rotated(axis,radians) * self.matrix;
        self.inverted = self.inverted * Matrix::rotated(axis,-radians);
        self
    }
}
