pub use super::super::math::{Mat3, Vec2};
pub use super::Geometries;

#[derive(Debug, Clone)]
pub struct Triangle2D {
  pub points: Vec<Vec2>,
  pub matrix: Mat3
}

impl Triangle2D {
  pub fn new() -> Geometries {
    Geometries::Triangle2D(Triangle2D { 
      points: vec![
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0)
      ],
      matrix: Mat3::identity()
    }) 
  }
}

impl Default for Triangle2D {
  fn default() -> Triangle2D {
    Triangle2D {
      points: vec![
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0),
        Vec2::from(0.0, 0.0)
      ],
      matrix: Mat3::identity()
    }
  }
}