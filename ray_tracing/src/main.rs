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
use texture::Texture;
use rayintersect::{RayIntersect, Intersect};
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, Vec3};
use std::time::Duration;
use color::Color;
use once_cell::sync::Lazy;
use std::sync::Arc;
use rayon::prelude::*;

const EPSILON: f32 = 1e-4;
const ZOOM:f32 = 0.5;
const SKYBOX_COLOR_NIGHT: (usize, usize, usize) = (4,12,36);
const SKYBOX_COLOR_DAY: (usize, usize, usize) = (135, 206, 235);
static STONE:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/dirt.png")));

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    
    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}



fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + intersect.normal * EPSILON; // Offset to avoid acne
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            shadow_intensity = 1.0;
            break;
        }
    }

    shadow_intensity
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube], lights: &[Light], depth:u32, is_day: bool) -> Color {
    if depth > 3 {
        return if is_day {Color::new(SKYBOX_COLOR_DAY.0 as u8, SKYBOX_COLOR_DAY.1 as u8, SKYBOX_COLOR_DAY.2 as u8)} else {Color::new(SKYBOX_COLOR_NIGHT.0 as u8, SKYBOX_COLOR_NIGHT.1 as u8, SKYBOX_COLOR_NIGHT.2 as u8)};
    }
    //println!("Casting ray from origin: {:?}, direction: {:?}", ray_origin, ray_direction);
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        //println!("Intersect result: {:?}", i);
        if i.is_intersecting && i.distance < zbuffer {
            intersect = i;
            zbuffer = intersect.distance;
        }
    }
    if !intersect.is_intersecting {
        //println!("No intersection. Returning background color.");
        return if is_day {Color::new(SKYBOX_COLOR_DAY.0 as u8, SKYBOX_COLOR_DAY.1 as u8, SKYBOX_COLOR_DAY.2 as u8)} else {Color::new(SKYBOX_COLOR_NIGHT.0 as u8, SKYBOX_COLOR_NIGHT.1 as u8, SKYBOX_COLOR_NIGHT.2 as u8)};
    }
    let ambient_light_factor = if is_day { 5.0 } else { 0.05 };  
    let ambient_light = intersect.material.albedo[0] * ambient_light_factor;
    let mut final_color = intersect.material.emission * intersect.material.emission_strength; 
    final_color = final_color + Color::new(ambient_light as u8, ambient_light as u8, ambient_light as u8); 
    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
        

        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
        let specular_intensity = view_dir
            .dot(&reflect_dir)
            .max(0.0)
            .powf(intersect.material.specular);

        let shadow = cast_shadow(&intersect, light, objects);
        let diffuse = intersect.material.get_diffuse(intersect.u, intersect.v)
            * diffuse_intensity
            * light.intensity
            * intersect.material.albedo[0]
            * (1.0 - shadow);

        let specular = light.color
            * specular_intensity
            * light.intensity
            * intersect.material.albedo[1]
            * (1.0 - shadow);
        
        
        final_color = (final_color + diffuse + specular);
    }

    let mut reflect_color = Color::new(0, 0, 0);
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point + intersect.normal * EPSILON;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth +1, is_day);

    }


    let mut refract_color = Color::new(0,0,0);
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = intersect.point - intersect.normal * EPSILON;
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, depth +1, is_day);
    }
    final_color * (1.0-reflectivity-transparency) + (reflect_color * reflectivity) + (refract_color * transparency)
    
}

pub fn render(framebuffer: &mut FrameBuffer, objects: &Vec<Cube>, camera: &mut Camera, lights: &[Light], is_day:bool) {
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

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0, is_day);
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
    //println!("Finished rendering.");
}


pub fn render_parallel(
    framebuffer: &mut FrameBuffer,
    objects: &[Cube],
    camera: &Camera,
    lights: &[Light],
    is_day: bool,
) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let aspect_ratio = width as f32 / height as f32;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();

    // Create a buffer to store colors computed in parallel
    let colors: Vec<Color> = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let x = i % width;
            let y = i / width;

            let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
            let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);

            // Cast the ray and get the color for the current pixel
            cast_ray(&camera.eye, &rotated_direction, objects, lights, 0, is_day)
        })
        .collect();

    // Write colors to framebuffer
    for (i, color) in colors.into_iter().enumerate() {
        let x = i % width;
        let y = i / width;
        framebuffer.set_current_color(color);
        framebuffer.point(x, y);
    }
}



//mode: se refiera a si queremos crear la cuadrícula sobre los ejex xy o yz
//grid_size: se refiera al tamaño que se quiere para la cuadrícula
pub fn create_grid(initial_min_position: &mut Vec3, initial_max_position: &mut Vec3, cube_length: f32, mode:usize, grid_size: usize, material:Material ) -> Vec<Cube>{
    let mut material_grid: Vec<Cube> = Vec::new();
    let mut current_min = *initial_min_position; // Dereference to get the actual value
    let mut current_max = *initial_max_position;
    let vertical_sum_vector = match mode {
        1 => Vec3::new(cube_length, 0.0, 0.0), //Cuando queremos usar la cuadrícula en xy
        2 => Vec3::new(cube_length, 0.0, 0.0),
        _ => Vec3::new(0.0, 0.0, 0.0) //Caso base
    };

    let horizontal_sum_vector = match mode {
        1 => Vec3::new(0.0, cube_length, 0.0), //Cuando queremos usar la cuadrícula en xy
        2 => Vec3::new(0.0, 0.0, cube_length),
        _ => Vec3::new(0.0, 0.0, 0.0) //Caso base
    };

    for i in 0..grid_size + 1 {

        for j in 0..grid_size +1 {
            material_grid.push(Cube { min: current_min, 
                max:current_max, 
                material: material.clone()});

            current_max += vertical_sum_vector;
            current_min += vertical_sum_vector;
        }
        *initial_min_position -= horizontal_sum_vector;
        *initial_max_position -= horizontal_sum_vector;
        current_min = *initial_min_position;
        current_max = * initial_max_position;
        


    }

    material_grid
}



// Function to parallelize the initial ray casting for each pixel

fn main() {
    let window_height = 600;
    let window_width = 800;

    let framebuffer_height = 600;
    let framebuffer_width = 800;


    let mut test = Material::material_with_texture(Color::new(128,128,128), 2.0, [0.5, 0.5, 0.0, 0.0], Some(STONE.clone()), 1.0, Color::new(0,0,0), 0.0);
    let mut test2 = Material::material_with_texture(Color::new(128,128,128), 9.0, [0.9, 0.5, 0.0, 0.0], Some(STONE.clone()), 1.0, Color::new(0,0,0), 0.0);
    let frame_delay = Duration::from_millis(0);
    let mut is_day = true;
    let mut camera = Camera::new(
        Vec3::new(-5.0, 5.0, -5.0), // Move the camera backward
        Vec3::new(0.0, 0.0, 0.0), //original: -0.5, -0.5, -1.0
        Vec3::new(0.0, 1.0, 0.0),
        false
    );
    let test_world = create_grid(&mut Vec3::new(-0.5, -0.5, -0.5),
    &mut Vec3::new(0.5, 0.5, 0.5),
      1.0, 2, 10, test);
    
    let objects = vec![
        Cube {
            min: Vec3::new(-0.5, -0.5, -0.5), 
            max: Vec3::new(0.5, 0.5, 0.5),
            material: Material {
                diffuse: Color::new(255, 0, 0),
                specular: 50.0,
                albedo: [0.9, 0.1, 0.0, 0.0],
                texture: None,
                refractive_index: 1.0,
                emission: Color::new(255, 0, 0),
                emission_strength: 2.0
            },
        },

        Cube {
            min: Vec3::new(3.0, -0.5, -1.0), // Move the cube forward
            max: Vec3::new(5.0, 0.0, 0.0),
            material: Material {
                diffuse: Color::new(0, 255, 0),
                specular: 265.0,
                albedo: [0.1, 0.9, 0.8, 0.5],
                texture: None,
                refractive_index: 1.0,
                emission: Color::new(0, 0, 0),
                emission_strength: 0.0
            },
            
        },

        Cube {
            min: Vec3::new(6.0, -0.5, -1.0), 
            max: Vec3::new(8.0, 0.5, 0.0),
            material: Material {
                diffuse: Color::new(255, 255, 0),
                specular: 9.0,
                albedo: [0.1, 0.9, 0.3, 0.4],
                texture:None,
                refractive_index: 1.0,
                emission: Color::new(0, 0, 0),
                emission_strength: 0.0
            },
            
        },
        Cube {

            min: Vec3::new(3.0, 2.0, -5.0), // Move the cube forward
            max: Vec3::new(5.0, 4.0, -3.0),
            material: Material {
                diffuse: Color::new(254, 138, 24),
                specular: 265.0,
                albedo: [0.1, 0.9, 0.4, 0.5],
                texture: None,
                refractive_index: 1.0,
                emission: Color::new(0, 0, 0),
                emission_strength: 0.0
            },

        }
    ];

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(Color::new(128,128,128));

    let rotation_speed = PI/60.0;

    let lights = [
        Light::new(Vec3::new(7.0, 5.0, 5.0), Color::new(255, 255, 255), 1.0),
        //Light::new(Vec3::new(7.0, 45.0, 6.0), Color::new(255, 255, 255), 10.0),
        //Light::new(Vec3::new(-3.0, 5.0, 5.0), Color::new(255, 255, 255), 10.0),
    ];
    render_parallel(&mut framebuffer, &test_world, &mut camera, &lights, is_day);
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
        if window.is_key_down(Key::W){
            camera.zoom(ZOOM);
        }
        if window.is_key_down(Key::S){
            camera.zoom(-ZOOM);
        }
        if window.is_key_down(Key::N){
            is_day = !is_day;
            camera.has_changed = true;
        }

        if camera.has_changed {
            render_parallel(&mut framebuffer, &test_world, &mut camera, &lights, is_day);
        }
        

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
