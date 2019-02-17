
use std::ops::{Add,Sub,Mul,Div};

//////////////////
//BaseVector
//////////////////
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct BaseVector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl BaseVector {

    pub fn new(x: f64, y: f64, z: f64) -> BaseVector {
        BaseVector{x,y,z}
    }
    pub fn from_value(value: f64) -> BaseVector { BaseVector{x: value, y: value, z: value} }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn dot(&self, other: &BaseVector) -> f64 {
        (*self**other).sum()
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