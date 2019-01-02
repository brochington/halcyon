use std::collections::HashMap;
use uuid::Uuid;
use crate::primitive_2d::{ Primitives2D };
use crate::texture::{ Textures };

#[derive(Debug, Clone)]
pub struct Scene2D {
  pub primitives: Vec<Primitives2D>, 
  pub textures: HashMap<Uuid, Textures>
}

impl Scene2D {
  pub fn new() -> Scene2D {
    Scene2D {
      primitives: Vec::new(),
      textures: HashMap::new()
    }
  }

  pub fn add_primitive(&mut self, primitive: Primitives2D) {
    self.primitives.push(primitive);
  }

  pub fn add_texture(&mut self, texture: Textures) {
    match texture {
      Textures::ColorTexture(color_texture) => {
        self.textures.insert(color_texture.get_id(), Textures::ColorTexture(color_texture));
      },
      Textures::ImageTexture(image_texture) => {
        self.textures.insert(image_texture.get_id(), Textures::ImageTexture(image_texture));
      }
    }
  }
}
