use crate::utils::color::Color;
use crate::texture::{ Textures };

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ColorTexture {
  colors: Vec<Color>,
  pub id: Uuid
}

impl ColorTexture {
  pub fn new() -> ColorTexture {
    ColorTexture {
      colors: vec![
        Color::new_rgba(1.0, 0.0, 0.0, 1.0),
        Color::new_rgba(0.0, 1.0, 0.0, 1.0),
        Color::new_rgba(0.0, 0.0, 1.0, 1.0),
      ],
      id: Uuid::new_v4()
    }
  }

  pub fn get_id(&self) -> Uuid {
    self.id.clone()
  }

  pub fn get_colors(&self) -> Vec<Color> {
    self.colors.clone()
  }

  pub fn wrap(texture: ColorTexture) -> Textures {
    Textures::ColorTexture(texture)
  }
}