use nalgebra_glm::{Vec3};
use crate::rayintersect::RayIntersect;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3, 
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, origin: &Vec3, ray_direction: &Vec3) -> bool {
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

        t_max >= 0.0 && t_min <= t_max
    }
}
