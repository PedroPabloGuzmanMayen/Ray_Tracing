use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
  pub diffuse: Color,
}

impl Material {
  pub fn new(
    diffuse: Color,
  ) -> Self {
    Material {
      diffuse,
    }
  }

  pub fn black() -> Self {
    Material {
      diffuse: Color::new(0, 0, 0),
    }
  }
}