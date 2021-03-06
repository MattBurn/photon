use cgmath::Vector3;
use cgmath::prelude::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::drawable::{
    Hit,
    Drawable
};

pub struct Sphere<M>
where
    M: Material
{
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: M
}

impl<M> Sphere<M>
where
    M: Material
{
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M> Drawable for Sphere<M> 
where
    M: Material
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            // negative root
            let result = (-b - discriminant.sqrt())/a;
            if result < t_max && result > t_min {
                return Some(Hit::new(
                    result,
                    ray.point_at_parameter(result),
                    (ray.point_at_parameter(result) - self.center)/self.radius,
                    &self.material,
                ))
            }
            // positive root
            let result = (-b + discriminant.sqrt())/a;
            if result < t_max && result > t_min {
                return Some(Hit::new(
                    result,
                    ray.point_at_parameter(result),
                    (ray.point_at_parameter(result) - self.center)/self.radius,
                    &self.material,
                ))
            }
        }
        None
    }
}