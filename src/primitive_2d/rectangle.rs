use uuid::Uuid;
use crate::math::{ Mat3, Vec2 };
use crate::primitive_2d::Primitives2D;

#[derive(Debug, Clone)]
pub struct Rectangle2D {
  pub width: f64,
  pub height: f64,
  pub transform: Mat3,
  pub texture_id: Uuid,
  pub uv_map: Vec<Vec2>
}

impl Rectangle2D {
  pub fn new() -> Rectangle2D {
    Rectangle2D {
      height: 100.0,
      width: 100.0,
      transform: Mat3::identity(),
      texture_id: Uuid::new_v4(),
      uv_map: vec![
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
      ]
    }
  }

  pub fn set_dimensions(&mut self, width: f64, height: f64) {
    self.width = width;
    self.height = height;
  }

  pub fn set_translation(&mut self, x: f64, y: f64) {
    self.transform = Mat3::set_translation(self.transform, x, y);
  }

  pub fn wrap(rect: Rectangle2D) -> Primitives2D {
    Primitives2D::Rectangle2D(rect)
  }

  pub fn set_texture_id(&mut self, id: Uuid) {
    self.texture_id = id;
  }
}