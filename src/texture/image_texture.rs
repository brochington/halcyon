use crate::utils::color::{ ColorU8, Color };
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ImageTexture {
  pub pixels: Vec<ColorU8>,
  pub height: u32, 
  pub width: u32,
  id: Uuid
}

impl ImageTexture {
  pub fn new() -> ImageTexture {
    ImageTexture {
      pixels: vec![],
      height: 0,
      width: 0,
      id: Uuid::new_v4()
    }
  }

  pub fn get_id(&self) -> Uuid {
    self.id.clone()
  }

  pub fn set_pixels(&mut self, pixels: Vec<ColorU8>) {
    self.pixels = pixels;
  }
}