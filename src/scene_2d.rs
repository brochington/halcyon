use crate::geometry::Geometries;

#[derive(Debug, Clone)]
pub struct Scene2D {
  pub geometries: Vec<Geometries>
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
