use crate::internal_state::InternalState;

#[derive(Debug)]
pub enum InternalActions {
  Stop,
  CursorMoved{x: f64, y: f64},
}

fn stop(state: &InternalState) -> InternalState {
  let mut new_state = state.clone();
  new_state.playing = false;
  new_state
}

// Not used yet
fn cursor_moved(state: &InternalState, _x: f64, _y: f64) -> InternalState{
  state.clone()
}

pub fn root_reducer(state: &InternalState, action: InternalActions) -> InternalState {
  match action {
    InternalActions::Stop => stop(&state),
    InternalActions::CursorMoved{ x, y } => cursor_moved(&state, x, y),
  }
}