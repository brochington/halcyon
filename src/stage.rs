use crate::scene_2d::Scene2D;
// use crate::state_management::{ Store };
// use crate::app_state::{ AppState };

pub trait Stage {
  fn new() -> Self;

  fn add_scene_2d(&mut self, scene: Scene2D) {}

  // fn play<F>(&mut self, cb: F, store: Store<AppState, Actions>) where F: Fn() {}

  fn stop(&mut self) {}
}
