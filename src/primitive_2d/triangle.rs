use uuid::Uuid;
pub use super::super::math::{ Mat3, Vec2 };
pub use super::{ Primitives2D, Primitive2D };


#[derive(Debug, Clone)]
pub struct Triangle2D {
  pub vertices: Vec<Vec2>,
  pub transform: Mat3,
  pub texture_id: Uuid,
  pub uv_map: Vec<Vec2>,
}

impl Triangle2D {
  pub fn new() -> Triangle2D {
    Triangle2D { 
      vertices: vec![
        Vec2::new(0.0, 0.0), 
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0)
      ],
      transform: Mat3::identity(),
      texture_id: Uuid::new_v4(),
      uv_map: vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 0.0)
      ]
    }
  }

  pub fn wrap(triangle: Triangle2D) -> Primitives2D {
    Primitives2D::Triangle2D(triangle)
  }

  pub fn set_vertices(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    self.vertices = vec![
      Vec2::new(x1, y1),
      Vec2::new(x2, y2),
      Vec2::new(x3, y3),
    ];
  }

  pub fn set_translation(&mut self, x: f64, y: f64) {
    self.transform = Mat3::set_translation(self.transform, x, y);
  }

  pub fn set_texture_id(&mut self, id: Uuid) {
    self.texture_id = id;
  }
}

impl Primitive2D for Triangle2D {}

impl Default for Triangle2D {
  fn default() -> Triangle2D {
    Triangle2D {
      vertices: vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0)
      ],
      transform: Mat3::identity(),
      texture_id: Uuid::new_v4(),
      uv_map: vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 0.0)
      ]
    }
  }
}