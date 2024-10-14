use nalgebra_glm::{Vec3};
use crate::rayintersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(
            1.0 / ray_direction.x,
            1.0 / ray_direction.y,
            1.0 / ray_direction.z,
        );

        let t1 = (self.min.x - origin.x) * inv_dir.x;
        let t2 = (self.max.x - origin.x) * inv_dir.x;
        let t3 = (self.min.y - origin.y) * inv_dir.y;
        let t4 = (self.max.y - origin.y) * inv_dir.y;
        let t5 = (self.min.z - origin.z) * inv_dir.z;
        let t6 = (self.max.z - origin.z) * inv_dir.z;

        let t_min = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let t_max = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if t_max < 0.0 || t_min > t_max {
            return Intersect::empty();
        }

        let distance = if t_min >= 0.0 { t_min } else { t_max };

        let hit_point = origin + ray_direction * distance;
        let normal = self.compute_normal(hit_point);

        Intersect::new(hit_point, normal, distance, self.material)
    }
}

impl Cube {
    /// Computes the normal at the intersection point based on which face was hit.
    fn compute_normal(&self, hit_point: Vec3) -> Vec3 {
        let epsilon = 1e-6;
        if (hit_point.x - self.min.x).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0);
        }
        if (hit_point.x - self.max.x).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0);
        }
        if (hit_point.y - self.min.y).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0);
        }
        if (hit_point.y - self.max.y).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0);
        }
        if (hit_point.z - self.min.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0);
        }
        if (hit_point.z - self.max.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, 1.0);
        }
        Vec3::new(0.0, 0.0, 0.0) // Default normal (should not happen in practice)
    }
}

