use nalgebra_glm::{Vec3};
use crate::rayintersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn get_uv(&self, hit_point: Vec3, normal: Vec3) -> (f32, f32) {
        // Debug print
   
        
        // Calculate dimensions of the cube
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;
        let depth = self.max.z - self.min.z;
        
        // Calculate local coordinates
        let local_x = (hit_point.x - self.min.x) / width;
        let local_y = (hit_point.y - self.min.y) / height;
        let local_z = (hit_point.z - self.min.z) / depth;
        
        
        
        let (u, v) = match (normal.x.round() as i32, 
                            normal.y.round() as i32, 
                            normal.z.round() as i32) {
            (1, 0, 0) => {  // Positive X face
                (local_z, local_y)
            },
            (-1, 0, 0) => { // Negative X face
                (1.0 - local_z, local_y)
            },
            (0, 1, 0) => {  // Positive Y face
                (local_x, local_z)
            },
            (0, -1, 0) => { // Negative Y face
                (local_x, 1.0 - local_z)
            },
            (0, 0, 1) => {  // Positive Z face
                (1.0 - local_x, local_y)
            },
            (0, 0, -1) => { // Negative Z face
                (local_x, local_y)
            },
            _ => {
                (0.0, 0.0)
            }
        };
        
    
        
        let final_u = u.clamp(0.0, 1.0);
        let final_v = v.clamp(0.0, 1.0);
        
        
        (final_u, final_v)
    }
    
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
        let epsilon = 1e-4;
        let is_on_surface = 
            (hit_point.x - self.min.x).abs() < epsilon || 
            (hit_point.x - self.max.x).abs() < epsilon ||
            (hit_point.y - self.min.y).abs() < epsilon || 
            (hit_point.y - self.max.y).abs() < epsilon ||
            (hit_point.z - self.min.z).abs() < epsilon || 
            (hit_point.z - self.max.z).abs() < epsilon;
        
        if !is_on_surface {
        }
        
        let(u,v) = self.get_uv(hit_point, normal);
        Intersect::new(hit_point, normal, distance, self.material.clone(), u, v)
    }
}

impl Cube {
    /// Computes the normal at the intersection point based on which face was hit.
    fn compute_normal(&self, hit_point: Vec3) -> Vec3 {
        let epsilon = 1e-6;
        
        // Check each face
        if (hit_point.x - self.min.x).abs() < epsilon {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (hit_point.x - self.max.x).abs() < epsilon {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (hit_point.y - self.min.y).abs() < epsilon {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (hit_point.y - self.max.y).abs() < epsilon {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (hit_point.z - self.min.z).abs() < epsilon {
            Vec3::new(0.0, 0.0, -1.0)
        } else if (hit_point.z - self.max.z).abs() < epsilon {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            // If we're here, something's wrong - print debug info
            Vec3::new(0.0, 1.0, 0.0)  // Default to up vector
        }
    }
}

