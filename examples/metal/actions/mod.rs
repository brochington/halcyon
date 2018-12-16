use crate::app_state::AppState;

pub enum Actions {
  // update_triangle(f64),
  Stop
}

// fn update_triangle<S: Clone>(state: S, num: f64) -> S {
//   state
// }

fn stop_playing(state: &AppState) -> AppState {

  println!("Stop playing!!!");
  let mut new_state = state.clone();
  new_state.playing = false;
  new_state
}

pub fn root_reducer(state: &AppState, action: Actions) -> AppState {
  // let mut new_state: AppState = state.clone();

  match action {
    // Actions::update_triangle(n) => update_triangle(&state, n),
    Actions::Stop => stop_playing(&state)
  }
}
