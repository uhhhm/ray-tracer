use crate::ray::{Ray};
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;

pub struct HittableList{
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList{
    pub fn new() -> Self{
        HittableList{objects: Vec::new()}
    }

    pub fn push(&mut self, object: impl Hittable + 'static){
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let mut best = None;
        let mut closest_so_far = t_max;
        for object in &self.objects{
            if let Some(rec) = object.hit(ray, t_min, closest_so_far){
                closest_so_far = rec.t;
                best = Some(rec);
            }
        }
        best
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::vec3::{Point3, Vec3};
    use crate::sphere::Sphere;
    use std::f64::INFINITY;

    
    #[test]
    fn test_nearest_hit(){
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut world = HittableList::new();
        world.push(Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.5));
        world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));

        let result = world.hit(&ray, 0.0, INFINITY);
        assert!((result.unwrap().t - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_empty_list(){
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let world = HittableList::new();

        assert_eq!(world.hit(&ray, 0.0, INFINITY), None);
    }

    #[test]
    fn test_first_miss_second_hit(){
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut world = HittableList::new();
        world.push(Sphere::new(Point3::new(0.0, 5.0, -1.0), 0.5));
        world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));

        let result = world.hit(&ray, 0.0, INFINITY);
        assert!((result.unwrap().t - 0.5).abs() < 1e-9);
    }
}
