pub use super::super::math::{Mat3, Vec2};
pub use super::Geometries;

#[derive(Debug, Clone)]
pub struct Square2D {
  points: Vec<Vec2>,
  matrix: Mat3,
}

impl Square2D {
  pub fn new() -> Geometries {
    Geometries::Square2D(Square2D {
      points: vec![
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0)
      ],
      matrix: Mat3::identity()
    })
  }
}

impl Default for Square2D {
  fn default() -> Square2D {
    Square2D {
      points: vec![
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0)
      ],
      matrix: Mat3::identity()
    }
  }
}

pub trait AddSquare2D {
  fn add(&mut self, square2D: Square2D) {}
}