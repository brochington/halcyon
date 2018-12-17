use crate::app_state::AppState;
use crate::scene_2d::Scene2D;

pub enum Actions {
  SetScene(Scene2D),
}

fn set_scene(state: &AppState, scene: Scene2D) -> AppState {
  let mut new_state = state.clone();
  new_state.scenes = vec![scene];
  new_state
}

pub fn root_reducer(state: &AppState, action: Actions) -> AppState {
  match action {
    Actions::SetScene(s) => set_scene(state, s),
  }
}
