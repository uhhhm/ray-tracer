use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Add for Vec3{
    type Output = Vec3;
    fn add(self, other:Vec3) -> Vec3{
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }       
}

impl Sub for Vec3{
    type Output = Vec3;
    fn sub(self, other:Vec3) -> Vec3{
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }     
}

impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, other:f64) -> Vec3{
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }     
}

impl Mul for Vec3{ // element wise multiplication, hadamard product
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3{
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }     
}

impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, other:f64) -> Vec3{
        self * (1.0/other)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Vec3{
    pub fn length(&self) -> f64{
        let num = self.x*self.x+self.y*self.y+self.z*self.z;
        num.sqrt()
    }

    pub fn length_squared(&self) -> f64{
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    pub fn new(x: f64, y: f64, z:f64) -> Self{
        Vec3 {x, y, z}
    }

    pub fn dot(&self, other: &Vec3) -> f64{
        self.x * other.x+self.y * other.y+self.z * other.z   
    }

    pub fn cross(&self, other: &Vec3) -> Vec3{
        Vec3::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }

    pub fn normalised(&self) -> Vec3{
        *self / self.length()
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_addition(){
        let a = Vec3::new(1.0, 3.0, 6.0);
        let b = Vec3::new(2.0, -1.0, 19.0);
        assert_eq!(a+b, Vec3::new(3.0, 2.0, 25.0));
    }

    #[test]
    fn test_cross(){
        let a = Vec3::new(1.1, 3.0, 6.0);
        let b = Vec3::new(2.0, -1.0, 19.190);
        let result = a.cross(&b);
        let expected = Vec3::new(63.57, -9.109, -7.1);
        let eps = 1e-9;
        assert!((result.x - expected.x).abs() < eps);
        assert!((result.y - expected.y).abs() < eps);
        assert!((result.z - expected.z).abs() < eps);
    }

    #[test]
    fn test_normalised(){
        let a = Vec3::new(2.0, -1.0, 19.0);
        assert_eq!(a.normalised(), Vec3::new(
            2.0 / (366.0_f64).sqrt(),
            -1.0 / (366.0_f64).sqrt(),
            19.0 / (366.0_f64).sqrt()))
    }
}