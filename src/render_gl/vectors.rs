use std::ops;

pub enum VecError {
    NotNormalized,
}
#[derive(Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Vec2 {
    fn from(other: (f32, f32)) -> Self {
        let (x, y) = other;
        Self {
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(other: (f32, f32, f32)) -> Self {
        let (x, y, z) = other;
        Self {
            x,
            y,
            z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        self + -rhs
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        -rhs + self
    }
}

impl Vec3 {
    pub fn magnitude(&self) -> f32 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;

        (x2 + y2 + z2).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();

        self / magnitude
    }

    pub fn dot(&self, other: Self) -> Result<f32, VecError> {
        if self.magnitude() != 1.0 || other.magnitude() != 1.0 {
            return Err(VecError::NotNormalized);
        }

        Ok((self.x * other.x) + (self.y * other.y) + (self.z * other.z))
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        let (x, y, z, w) = value;
        Self {
            x,
            y,
            z,
            w,
        }
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        let Vec3 { x, y, z } = value;
        Self {
            x,
            y,
            z,
            w: 1.0,
        }
    }
}
