use crate::math::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
  pub values: [f64; 9]
}

impl Mat3 {
  pub fn identity() -> Mat3 {
    Mat3 {
      values: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
    }
  }

  pub fn set_translation(mut mat3: Mat3, x: f64, y: f64) -> Mat3 {
    mat3.values[6] = x;
    mat3.values[7] = y;
    mat3
  }

  pub fn get_translation(&self) -> [f64; 2] {
    [ self.values[6], self.values[7] ]
  }
}