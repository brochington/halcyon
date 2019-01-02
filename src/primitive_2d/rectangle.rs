use crate::math::{ Mat3 };
use crate::primitive_2d::Primitives2D;

#[derive(Debug, Clone)]
pub struct Rectangle2D {
  pub width: f64,
  pub height: f64,
  pub transform: Mat3,
}

impl Rectangle2D {
  pub fn new() -> Rectangle2D {
    Rectangle2D {
      height: 100.0,
      width: 100.0,
      transform: Mat3::identity(),
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
}