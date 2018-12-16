use crate::internal_state::InternalState;
use std::time::{ Instant };

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

fn cursor_moved(state: &InternalState, x: f64, y: f64) -> InternalState{
  println!("cursor: {}, {}", x, y);
  let now = Instant::now();
  // let new_state = state.clone();
  // println!("{}", now.elapsed().subsec_nanos());
  // new_state
  state.clone()
}

pub fn root_reducer(state: &InternalState, action: InternalActions) -> InternalState {
  match action {
    InternalActions::Stop => stop(&state),
    InternalActions::CursorMoved{ x, y } => cursor_moved(&state, x, y)
  }
}