use crate::color::Color;
use once_cell::sync::Lazy;
use std::sync::Arc;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Material {
  pub diffuse: Color,
  pub specular: f32,
  pub albedo: [f32;4],
  pub texture: Option<Arc<Texture>>,
  pub refractive_index: f32,
  pub emission: Color,           // New: emission color
  pub emission_strength: f32
}

impl Material {
  pub fn new(
    diffuse: Color,
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
    emission: Color,           // New: emission color
    emission_strength: f32
  ) -> Self {
    Material {
      diffuse,
      specular,
      albedo,
      texture:None,
      refractive_index,
      emission,
      emission_strength
    }
  }

  pub fn material_with_texture( 
    diffuse: Color,
    specular: f32,
    albedo: [f32; 4],
    texture: Option<Arc<Texture>>,
    refractive_index: f32,
    emission: Color,           // New: emission color
    emission_strength: f32 ) -> Self {
      Material {
        diffuse,
        specular,
        albedo,
        texture,
        refractive_index,
        emission,
        emission_strength
      }
    }


    pub fn get_diffuse(&self, u: f32, v: f32) -> Color {
      if let Some(texture) = &self.texture {
          let x = ((u * texture.width as f32).round() as u32).min(texture.width - 1);
          let y = ((v * texture.height as f32).round() as u32).min(texture.height - 1);
          texture.get_pixel_color(x, y)
      } else {
          self.diffuse
      }
  }


  pub fn black() -> Self {
    Material {
      diffuse: Color::new(0, 0, 0),
      specular: 0.0,
      albedo: [0.0, 0.0, 0.0, 0.0],
      texture: None,
      refractive_index: 0.0,
      emission: Color::new(0, 0, 0),
      emission_strength: 0.0
    }
  }
}