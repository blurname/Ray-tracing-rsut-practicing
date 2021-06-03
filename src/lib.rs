use std::ops::{self};
#[derive(Debug, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}
// impl ops::MulAssign<f64> for Vec3 {
//     pub fn mul_assign(&mut self, rhs: f64) {
//         self.x *= rhs;
//         self.y *= rhs;
//         self.z *= rhs;
//     }
// }
// impl ops::AddAssign for Vec3 {
//     pub fn add_assign(&mut self, rhs: Vec3) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//         self.z += rhs.z;
//     }
// }
// impl ops::DivAssign<f64> for Vec3 {
//     fn div_assign(&mut self, rhs: f64) {
//         self * (1.0 / rhs)
//     }
// }

// Vec3 + Vec3
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Vec3 - Vec3
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
// Vec3 * Vec3
impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// Vec3 * f64
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
// f64 * Vec3
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        rhs * self
    }
}
// Vec3 / f64
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * u.z - u.z * v.y,
        y: u.z * u.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    u / u.length()
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[test]
fn test() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    let c = &a + &b;
    let d = &a - &b;
    let e = &a / 10.0;
    let a_add = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(dot(&a, &b), 6.0);
    // println!("{:?}", ac + ab)
    assert_eq!(c, Vec3::new(2.0, 3.0, 4.0));
    assert_eq!(e.x(), 0.1);
    // assert_eq!(ac + ab, Vec3::new(2.0, 3.0, 4.0));
}

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction,
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        &self.orig + &(t * &self.dir)
    }
}
#[test]
fn testRay() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    let ray = Ray::new(&a, &b);
    let c = ray.origin();
    let d = ray.direction();
    let e = ray.at(2.0);
    assert_eq!(&a.x(), &c.x());
    assert_eq!(&b.x(), &d.x());
    assert_eq!(e.y(), 4.0);
}
