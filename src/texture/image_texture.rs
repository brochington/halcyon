use crate::utils::color::{ Color, ColorModels };
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ImageTexture {
  pub pixels: Vec<Color>,
  pub height: u32, 
  pub width: u32,
  id: Uuid
}

impl ImageTexture {
  pub fn new() -> ImageTexture {
    ImageTexture {
      pixels: vec![
        Color {
          model: ColorModels::RGBA,
          color: vec![1.0, 0.0, 0.0, 1.0],
        },
        Color {
          model: ColorModels::RGBA,
          color: vec![0.0, 1.0, 0.0, 1.0],
        },
        Color {
          model: ColorModels::RGBA,
          color: vec![0.0, 0.0, 1.0, 1.0],
        },
        Color {
          model: ColorModels::RGBA,
          color: vec![0.5, 0.5, 0.7, 1.0],
        },
      ],
      height: 0,
      width: 0,
      id: Uuid::new_v4()
    }
  }

  pub fn get_id(&self) -> Uuid {
    self.id.clone()
  }
}