use crate::vec3::{Point3, Vec3};
use crate::ray::{Ray};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord{
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn new(ray: &Ray, t: f64, outward_normal: Vec3) -> HitRecord{
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face{outward_normal} else {-outward_normal};
        HitRecord{point: ray.at(t), normal, t, front_face}
    }
}