use crate::utils::color::{ColorF32, Color};
use crate::texture::{ Textures };

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ColorTexture {
  colors: Vec<ColorF32>,
  pub id: Uuid
}

impl ColorTexture {
  pub fn new() -> ColorTexture {
    ColorTexture {
      colors: vec![
        ColorF32::from_rgba(1.0, 0.0, 0.0, 1.0),
        ColorF32::from_rgba(0.0, 1.0, 0.0, 1.0),
        ColorF32::from_rgba(0.0, 0.0, 1.0, 1.0),
        ColorF32::from_rgba(1.0, 0.0, 0.0, 1.0),
        ColorF32::from_rgba(0.0, 1.0, 0.0, 1.0),
        ColorF32::from_rgba(0.0, 0.0, 1.0, 1.0),
      ],
      id: Uuid::new_v4()
    }
  }

  pub fn get_id(&self) -> Uuid {
    self.id.clone()
  }

  pub fn get_colors(&self) -> Vec<ColorF32> {
    self.colors.clone()
  }

  pub fn wrap(texture: ColorTexture) -> Textures {
    Textures::ColorTexture(texture)
  }
}