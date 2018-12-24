
use std::ops::{Mul};
use std::fmt;

use super::{VectorTrait, Point, Vector};

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix {
    base: [[f64; 4]; 4]
}

#[derive(Copy, Clone,Debug)]
pub enum RotationAxis{
    Xaxis,
    Yaxis,
    Zaxis
}

impl Matrix {
    pub fn zeros() -> Matrix {
        Matrix{ base: [[0.0; 4]; 4] }
    }

    pub fn identiy() -> Matrix {
        let mut mat = Self::zeros();
        for i in 0..4 {
            mat.base[i][i] = 1.0;
        }
        mat
    }

    pub fn translated(vector: Vector) -> Matrix {
        let mut new = Self::identiy();
        new.base[0][3] = vector.x();
        new.base[1][3] = vector.y();
        new.base[2][3] = vector.z();
        new
    }

    pub fn scaled(x: f64, y: f64, z: f64) -> Matrix {
        let mut new = Self::identiy();
        new.base[0][0] = x;
        new.base[1][1] = y;
        new.base[2][2] = z;
        new
    }

    pub fn rotated(axis: RotationAxis, radians: f64) -> Matrix{
        let mut new = Self::identiy();
        let cos = radians.cos();
        let sin = radians.sin();
        let (first, second) = match axis {
            RotationAxis::Xaxis => (1, 2),
            RotationAxis::Yaxis => (2, 0),
            RotationAxis::Zaxis => (0, 1)
        };

        new.base[first][first] = cos;
        new.base[second][second] = cos;
        new.base[first][second] = -sin;
        new.base[second][first] = sin;
        new
    }

    pub fn get(&self, n: usize, m: usize) -> f64 {
        self.base[n][m]
    }

    pub fn transpose(mut self) -> Matrix {
        let swap = |base: &mut [[f64; 4]; 4], i: usize, j: usize| {
            let value = base[i][j];
            base[i][j] = base[j][i];
            base[j][i] = value;
        };

        swap(&mut self.base, 1, 0);
        swap(&mut self.base, 2, 0);
        swap(&mut self.base, 2, 1);
        swap(&mut self.base, 3, 0);
        swap(&mut self.base, 3, 1);
        swap(&mut self.base, 3, 2);
        self
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[{}, {}, {}, {}\n {}, {}, {}, {}\n {}, {}, {}, {}\n {}, {}, {}, {}]", self.base[0][0], self.base[0][1], self.base[0][2], self.base[0][3], self.base[1][0], self.base[1][1], self.base[1][2], self.base[1][3], self.base[2][0], self.base[2][1], self.base[2][2], self.base[2][3], self.base[3][0], self.base[3][1], self.base[3][2], self.base[3][3])
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut mat = Self::zeros();
        for i in 0..4 {
            for j in 0..4 {
                mat.base[i][j] = self.base[i][0]*rhs.base[0][j] + self.base[i][1]*rhs.base[1][j] + self.base[i][2]*rhs.base[2][j] + self.base[i][3]*rhs.base[3][j];
            }
        }
        mat
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        let base = rhs.base();
        let x = self.base[0][0]*base.x + self.base[0][1]*base.y + self.base[0][2]*base.z;
        let y = self.base[1][0]*base.x + self.base[1][1]*base.y + self.base[1][2]*base.z;
        let z = self.base[2][0]*base.x + self.base[2][1]*base.y + self.base[2][2]*base.z;
        Vector::new(x,y,z)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let base = rhs.base();
        let w = self.base[3][0]*base.x + self.base[3][1]*base.y + self.base[3][2]*base.z + self.base[3][3];
        let x = (self.base[0][0]*base.x + self.base[0][1]*base.y + self.base[0][2]*base.z + self.base[0][3]) / w;
        let y = (self.base[1][0]*base.x + self.base[1][1]*base.y + self.base[1][2]*base.z + self.base[1][3]) / w;
        let z = (self.base[2][0]*base.x + self.base[2][1]*base.y + self.base[2][2]*base.z + self.base[2][3]) / w;
        Point::new(x,y,z)
    }
}