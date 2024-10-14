mod framebuffer;
mod color;
mod texture;
mod cube;
mod rayintersect;
mod material;
mod camera;
mod light;
use std::f32::consts::PI;
use camera::Camera;
use cube::Cube;
use material::Material;
use light::Light;
use framebuffer::FrameBuffer;
use rayintersect::{RayIntersect, Intersect};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, Vec3};
use std::time::Duration;
use color::Color;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube], light: &Light) -> Color {
    //println!("Casting ray from origin: {:?}, direction: {:?}", ray_origin, ray_direction);
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        //println!("Intersect result: {:?}", i);
        if i.is_intersecting && i.distance < zbuffer {
            intersect = i;
            zbuffer = i.distance;
        }
    }
    if !intersect.is_intersecting {
        //println!("No intersection. Returning background color.");
        return Color::new(4,12,36);
    }
    let hit_point = intersect.point;

    let light_dir = (light.position - hit_point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
    //println!("Light dir: {}", light_dir);

    //println!("Point normal: {}", intersect.normal);

    let diffuse_intensity = intersect.normal.dot(&light_dir);
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color*specular_intensity*light.intensity * intersect.material.albedo[1];
    //println!("Diffuse intensity: {}", diffuse_intensity);

    let mut diffuse = intersect.material.diffuse * diffuse_intensity * light.intensity * intersect.material.albedo[0];
    diffuse + specular
}

pub fn render(framebuffer: &mut FrameBuffer, objects: &[Cube], camera: &mut Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov/2.0).tan();
    //println!("Rendering... Camera position: {:?}", camera.eye);

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = &Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
    //println!("Finished rendering.");
}
fn main() {
    let window_height = 600;
    let window_width = 800;

    let framebuffer_height = 600;
    let framebuffer_width = 800;

    let frame_delay = Duration::from_millis(0);
    let mut camera = Camera::new(
        Vec3::new(-5.0, 5.0, -5.0), // Move the camera backward
        Vec3::new(-0.5, -0.5, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    
    let objects = [
        Cube {
            min: Vec3::new(-0.5, -0.5, -1.0), // Move the cube forward
            max: Vec3::new(0.0, 0.0, 0.0),
            material: Material {
                diffuse: Color::new(255, 0, 0),
                specular: 50.0,
                albedo: [0.9, 0.1]
            },
        },

        Cube {
            min: Vec3::new(3.0, -0.5, -1.0), // Move the cube forward
            max: Vec3::new(5.0, 0.0, 0.0),
            material: Material {
                diffuse: Color::new(0, 255, 0),
                specular: 265.0,
                albedo: [0.1, 0.9]
            },
            
        }
    ];

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(Color::new(128,128,128));

    let rotation_speed = PI/60.0;

    let light = Light::new(
        Vec3::new(7.0,0.0,5.0),
        Color::new(255,255,255),
        2.0
    );

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
        render(&mut framebuffer, &objects, &mut camera, &light);


        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
