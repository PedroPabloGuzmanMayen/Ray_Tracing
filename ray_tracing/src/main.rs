mod framebuffer;
mod color;
mod texture;
mod cube;
mod rayintersect;
use cube::Cube;
use framebuffer::FrameBuffer;
use rayintersect::RayIntersect;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, Vec3};
use std::time::Duration;
use color::Color;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube]) -> Color {
    for object in objects {
        if object.ray_intersect(ray_origin, ray_direction) {
            return Color::new(0, 0, 255);
        }
    }
    Color::new(0,0,0)
}

pub fn render(framebuffer: &mut FrameBuffer, objects: &[Cube]) {
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
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}
fn main() {
    let window_height = 600;
    let window_width = 800;

    let framebuffer_height = 600;
    let framebuffer_width = 800;

    let frame_delay = Duration::from_millis(0);

    let framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

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

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
