mod framebuffer;
mod color;
mod texture;
mod rayintersect;
mod sphere;
mod material;
use sphere::Sphere;
use framebuffer::FrameBuffer;
use rayintersect::{RayIntersect, Intersect};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, Vec3};
use std::time::Duration;
use color::Color;
use std::f32::consts::PI;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        // Get the intersection result
        let i = object.ray_intersect(ray_origin, ray_direction);

        // Check if there was an intersection and if it's closer than the previous one
        if i.is_intersecting && i.distance < zbuffer {
            closest_intersect = i;
            zbuffer = i.distance;
        }
    }

    // If no intersection was found, return background color
    if !closest_intersect.is_intersecting {
        return Color::new(4, 12, 36); // Background color
    }

    // Return the color of the intersected object's material
    closest_intersect.material.diffuse
}


pub fn render(framebuffer: &mut FrameBuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = &Vec3::new(screen_x, screen_y, -1.0).normalize();

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 5.0), &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}
fn main() {
    let window_height = 1000;
    let window_width = 1000;

    let framebuffer_height = 1000;
    let framebuffer_width = 1000;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let objects = [
    Sphere {
        center: Vec3::new(0.0, 0.0, -1.0), // Move this sphere back
        radius: 4.0,
        material: material::Material { diffuse: Color::new(111, 78, 55) }
    },
    Sphere {
        center: Vec3::new(0.0, 0.0, 3.0), // Move this sphere back as well
        radius: 0.5,
        material: material::Material { diffuse: Color::new(255, 0, 0) }
    },
    Sphere {
        center: Vec3::new(1.0, 0.5, 3.0), // Move this sphere back as well
        radius: 0.25,
        material: material::Material { diffuse: Color::new(0, 0, 0) }
    },
    Sphere {
        center: Vec3::new(-1.0, 0.5, 3.0), // Move this sphere back as well
        radius: 0.25,
        material: material::Material { diffuse: Color::new(0, 0, 0) }
    },
    Sphere {
        center: Vec3::new(-1.5, 1.5, 2.5), // Move this sphere back as well
        radius: 0.50,
        material: material::Material { diffuse: Color::new(111, 78, 55)  }
    },
    Sphere {
        center: Vec3::new(1.5, 1.5, 2.5), // Move this sphere back as well
        radius: 0.50,
        material: material::Material { diffuse: Color::new(111, 78, 55)  }
    },

];

    let mut window = Window::new(
        "Minecraft RayTracer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();
    
    while window.is_open() {
        if window.is_key_down(Key::Escape){
            break;
        }
        framebuffer.clear();
        render(&mut framebuffer, &objects);

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
