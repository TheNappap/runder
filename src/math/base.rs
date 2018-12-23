
use std::ops::{Add,Sub,Mul,Div};
use super::utils::VectorTrait;

//////////////////
//BaseVector
//////////////////
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BaseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl BaseVector {
    pub fn invert(mut self) -> BaseVector {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl VectorTrait for BaseVector {
    fn base(&self) -> &BaseVector {
        self
    }
}

impl Add<BaseVector> for BaseVector {
    type Output = BaseVector;

    fn add(mut self, rhs: BaseVector) -> BaseVector {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self
    }
}

impl Sub<BaseVector> for BaseVector {
    type Output = BaseVector;

    fn sub(mut self, rhs: BaseVector) -> BaseVector {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self
    }
}

impl Mul<BaseVector> for BaseVector {
    type Output = BaseVector;

    fn mul(mut self, rhs: BaseVector) -> BaseVector {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self
    }
}

impl Div<BaseVector> for BaseVector {
    type Output = BaseVector;

    fn div(mut self, rhs: BaseVector) -> BaseVector {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self
    }
}

impl Add<f64> for BaseVector{
    type Output = BaseVector;

    fn add(mut self, rhs: f64) -> BaseVector {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
        self
    }
}

impl Sub<f64> for BaseVector{
    type Output = BaseVector;

    fn sub(mut self, rhs: f64) -> BaseVector {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
        self
    }
}

impl Mul<f64> for BaseVector{
    type Output = BaseVector;

    fn mul(mut self, rhs: f64) -> BaseVector {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self
    }
}

impl Div<f64> for BaseVector{
    type Output = BaseVector;

    fn div(mut self, rhs: f64) -> BaseVector {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self
    }
}

impl Add<BaseVector> for f64{
    type Output = BaseVector;

    fn add(self, mut rhs: BaseVector) -> BaseVector {
        rhs.x = rhs.x + self;
        rhs.y = rhs.y + self;
        rhs.z = rhs.z + self;
        rhs
    }
}

impl Sub<BaseVector> for f64{
    type Output = BaseVector;

    fn sub(self, mut rhs: BaseVector) -> BaseVector {
        rhs.x = rhs.x - self;
        rhs.y = rhs.y - self;
        rhs.z = rhs.z - self;
        rhs
    }
}

impl Mul<BaseVector> for f64{
    type Output = BaseVector;

    fn mul(self, mut rhs: BaseVector) -> BaseVector {
        rhs.x = rhs.x * self;
        rhs.y = rhs.y * self;
        rhs.z = rhs.z * self;
        rhs
    }
}

impl Div<BaseVector> for f64{
    type Output = BaseVector;

    fn div(self, mut rhs: BaseVector) -> BaseVector {
        rhs.x = rhs.x / self;
        rhs.y = rhs.y / self;
        rhs.z = rhs.z / self;
        rhs
    }
}