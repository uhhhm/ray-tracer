use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray{
    pub origin:Point3,
    pub direction: Vec3
}

impl Ray{
    pub fn new(origin: Point3, direction: Vec3) -> Self{
        Ray{origin, direction}
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_mult_by_zero_returns_origin(){
        let a = Point3::new(1.0, 3.0, 6.0);
        let b = Vec3::new(2.0, -1.0, 19.0);
        let ray = Ray::new(a, b);
        assert_eq!(ray.at(0.0), Point3::new(1.0, 3.0, 6.0));
    }

    #[test]
    fn test_at(){
        let origin = Point3::new(2.0, 4.0, 7.0);
        let dir = Vec3::new(2.0, 0.0, -1.0);
        let ray = Ray::new(origin, dir);
        assert_eq!(ray.at(3.0), Point3::new(8.0, 4.0, 4.0));
    }

    #[test]
    fn test_at_neg_t(){
        let origin = Point3::new(2.0, 4.0, 7.0);
        let dir = Vec3::new(2.0, 0.0, -1.0);
        let ray = Ray::new(origin, dir);
        assert_eq!(ray.at(-4.0), Point3::new(-6.0, 4.0, 11.0));
    }
}