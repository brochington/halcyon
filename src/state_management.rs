use std::fmt;

pub struct Store <S: Clone, A> {
  state: S,
  reducer: fn(&S, A) -> S,
}

impl<S: Clone + fmt::Debug, A> fmt::Debug for Store<S, A> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "state: {:#?}", self.state)
  }
}

impl<S: Clone, A> Store<S, A> {
  pub fn init(
    reducer: fn(&S, A) -> S, 
    initial_state: S,
  ) -> Store<S, A> {
    Store {
      state: initial_state,
      reducer,
    }
  }

  pub fn get_state(&self) -> &S {
    &self.state
  }

  pub fn dispatch(&mut self, action: A) {
    self.state = (self.reducer)(&self.state, action);
  }
}