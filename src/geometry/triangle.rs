pub use crate::scene_2d::SceneAddable;
pub use super::super::math::{Mat3, Vec2};
pub use super::Geometries;

#[derive(Debug, Clone)]
pub struct Triangle2D {
  points: Vec<Vec2>,
  matrix: Mat3
}

impl SceneAddable for Triangle2D {}

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

pub trait AddTriangle2D {
  fn add(&mut self, triangle2D: Triangle2D) {}
}