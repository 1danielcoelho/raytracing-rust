use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn r(&self) -> f64 {
        return self.x;
    }

    pub fn g(&self) -> f64 {
        return self.y;
    }

    pub fn b(&self) -> f64 {
        return self.z;
    }

    pub fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    pub fn squared_length(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn normalize(&mut self) {
        let length = 1.0 / self.length();
        self.x = self.x * length;
        self.y = self.y * length;
        self.z = self.z * length;
    }

    pub fn normalized(&self) -> Vec3 {
        let length = 1.0 / self.length();
        return Vec3 {
            x: self.x * length,
            y: self.y * length,
            z: self.z * length,
        };
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        };
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: &'b Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl<'a, 'b> Div<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, other: &'b Vec3) -> Vec3 {
        return Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}
