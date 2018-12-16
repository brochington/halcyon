extern crate halcyon;
extern crate winit;

use crate::halcyon::{ Stage, StageConfig, MetalStage, Scene2D, geometry, state_management };
// use crate::geometry::Geometries;
use crate::geometry::triangle::{ Triangle2D };
use crate::geometry::square::{ Square2D };
// use crate::state_management::Store;
// use crate::actions::{ Actions, root_reducer };

fn main() {
  let mut stage = MetalStage::new();
  // let mut scene2d = Scene2D::new();

  // let tri = Triangle2D::new();
  // let sq = Square2D::new();
  // let initial_state = app_state::AppState {
  //   playing: true,
  //   geometries: vec![tri, sq]
  // };


  // let store = Store::init(root_reducer, initial_state);
  stage.play();
  // stage.play(|local_store| {
    
  // }, store);
}