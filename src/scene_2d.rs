use crate::geometry::Geometries;

pub trait SceneAddable {}

#[derive(Debug)]
pub struct Scene2D {
  geometries: Vec<Geometries>
}

impl Scene2D {
  pub fn new() -> Scene2D {
    Scene2D {
      geometries: Vec::new()
    }
  }

  pub fn add(&mut self, geometry: Geometries) {
    self.geometries.push(geometry);
  }
}