// sphere.rs

use nalgebra_glm::{Vec3};
use crate::rayintersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = origin - self.center;
        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        // Solve the quadratic equation a*t^2 + b*t + c = 0
        let discriminant = b * b - 4.0 * a * c;

        // If the discriminant is negative, no real roots (no intersection)
        if discriminant < 0.0 {
            return Intersect::empty();
        }

        // Check both solutions (t1 and t2) and take the closest positive one
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        // Determine the closest intersection point that's in front of the ray
        let t = if t1 >= 0.0 {
            t1
        } else if t2 >= 0.0 {
            t2
        } else {
            return Intersect::empty();
        };

        // Calculate the intersection point and normal
        let intersection_point = origin + ray_direction * t;
        let normal = (intersection_point - self.center).normalize();

        // Return the intersection details
        Intersect::new(intersection_point, normal, t, self.material)
    }
}

