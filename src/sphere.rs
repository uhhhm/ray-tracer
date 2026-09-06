use crate::vec3::{Point3, Vec3};
use crate::ray::{Ray};
use crate::hit_record::HitRecord;
use std::f64::INFINITY;

pub struct Sphere{
    pub center:Point3,
    pub radius:f64
}

fn vec_close(a: Vec3, b: Vec3) -> bool {
    (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9 && (a.z - b.z).abs() < 1e-9
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere{
        Sphere{center, radius}
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {return None;}
        let sqrtd = disc.sqrt();

        let root1 = (-half_b - sqrtd)/a;
        let root2 = (-half_b + sqrtd)/a;

        if !(root1 < t_min || root1 > t_max) {
            let hit_point = ray.at(root1);
            let outward_normal = (hit_point - self.center) / self.radius;
            return Some(HitRecord::new(ray, root1, outward_normal));
        }
        if !(root2 < t_min || root2 > t_max) {
            let hit_point = ray.at(root2);
            let outward_normal = (hit_point - self.center) / self.radius;
            return Some(HitRecord::new(ray, root2, outward_normal));
        }
        None
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn hit_test(){
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 0.0, INFINITY);

        assert!(result.unwrap().front_face);
        assert!(vec_close(result.unwrap().point, Vec3::new(0.0, 0.0, -0.5)));
        assert!((result.unwrap().t - 0.5).abs() < 1e-9);
        assert!(vec_close(result.unwrap().normal, Vec3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn miss_test(){
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 1.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 0.0, INFINITY);
        assert_eq!(result, None);
    }

    #[test]
    fn test_intersection_behind_ray_with_both_roots_negative(){
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5);

        let result = sphere.hit(&ray, 0.0, INFINITY);
        assert_eq!(result, None);
    }

    #[test]
    fn test_ray_starts_inside_sphere(){
        let origin = Point3::new(0.0, 0.0, -1.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 0.0, INFINITY);

        assert!(!result.unwrap().front_face);
        assert!((result.unwrap().t - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_t_max(){
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        let result1 = sphere.hit(&ray, 0.0, 0.4);
        assert_eq!(result1, None);
        
        let result2 = sphere.hit(&ray, 0.0, 0.6);
        assert!(result2.is_some());
    }

    #[test]
    fn test_t_min(){
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        let result1 = sphere.hit(&ray, 2.0, INFINITY);
        assert_eq!(result1, None);
        
        let result2 = sphere.hit(&ray, 0.6, INFINITY);
        assert!(result2.is_some());
    }
}