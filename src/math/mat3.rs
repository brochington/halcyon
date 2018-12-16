#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
  pub x: [f64; 3],
  pub y: [f64; 3],
  pub z: [f64; 3],
}

impl Mat3 {
  pub fn identity() -> Mat3 {
    Mat3 {
      x: [1.0, 0.0, 0.0],
      y: [0.0, 1.0, 0.0],
      z: [0.0, 0.0, 1.0]
    }
  }
}