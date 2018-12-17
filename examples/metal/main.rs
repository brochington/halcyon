extern crate halcyon;
extern crate winit;

pub mod app_state;
pub mod actions;


use crate::halcyon::{ Stage,  MetalStage, metal_stage, Scene2D, geometry, state_management, scene_2d, math };
use crate::geometry::triangle::{ Triangle2D };
use crate::geometry::Geometries;
use crate::state_management::Store;
use crate::actions::{ Actions, root_reducer };
use crate::metal_stage::play;
use crate::math::Vec2;

fn main() {
  let stage = MetalStage::new();
  let mut beginning_scene = Scene2D::new();
  let tri = Triangle2D::new();
  beginning_scene.add(tri);

  let initial_state = app_state::AppState {
    scenes: vec![beginning_scene]
  };

  let mut store = Store::init(root_reducer, initial_state);

  let on_update = move || {
    let current_state = store.get_state();
    let mut next_scene = current_state.scenes[0].clone();

    if let Geometries::Triangle2D(triangle) = &next_scene.geometries[0] {
      let mut next_tri = triangle.clone();

      next_tri.points[0] = Vec2 { 
        x: if triangle.points[0].x > 1.0 { -1.0 } else { triangle.points[0].x + 0.01 },
        y: -1.0
      };

      next_tri.points[1] = Vec2 { 
        x: if triangle.points[1].x < -1.0 { 1.0 } else { triangle.points[1].x - 0.01 },
        y: 1.0
      };

      next_tri.points[2] = Vec2 { 
        x: 1.0,
        y: if triangle.points[2].y > 1.0 { -1.0 } else { triangle.points[2].y + 0.01 },
      };

      next_scene.geometries[0] = Geometries::Triangle2D(next_tri);
    }

    store.dispatch(Actions::SetScene(next_scene.clone()));

    next_scene
  };

  play(stage, Box::new(on_update));
}