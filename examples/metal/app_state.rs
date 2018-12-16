use crate::geometry::Geometries;

#[derive(Debug, Default, Clone)]
pub struct AppState {
  pub playing: bool,
  pub geometries: Vec<Geometries>
}