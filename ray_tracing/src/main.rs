mod framebuffer;
mod color;
mod texture;
mod cube;
mod rayintersect;
mod material;
mod camera;
use std::f32::consts::PI;
use camera::Camera;
use cube::Cube;
use material::Material;
use framebuffer::FrameBuffer;
use rayintersect::{RayIntersect, Intersect};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, Vec3};
use std::time::Duration;
use color::Color;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            intersect = i;
            zbuffer = i.distance;
        }
    }
    if !intersect.is_intersecting {
        return Color::new(4,12,36);
    }

    let diffuse = intersect.material.diffuse;
    diffuse
}

pub fn render(framebuffer: &mut FrameBuffer, objects: &[Cube], camera: &mut Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio;

            let ray_direction = &Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects);
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
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(Color::new(128,128,128));

    let objects = [
    Cube {
        min: Vec3::new(-1.0, -1.0, 4.0),  // Moved forward to Z = 3.0
        max: Vec3::new(1.0, 1.0, 5.0),    // Adjusted to be fully in front of the camera
        material: Material {
            diffuse: Color::new(255, 0, 0),
        },
    },
    Cube {
        min: Vec3::new(-0.5, -0.5, 5.5),  // Placed near the camera
        max: Vec3::new(0.5, 0.5, 2.5),    // In front of the first cube
        material: Material {
            diffuse: Color::new(0, 255, 0),
        },
    },
];

    let rotation_speed = PI/60.0;

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
        if window.is_key_down(Key::Left){
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right){
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up){
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down){
            camera.orbit(0.0, rotation_speed);
        }
        framebuffer.clear();
        render(&mut framebuffer, &objects, &mut camera);


        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
