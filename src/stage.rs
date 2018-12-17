pub trait Stage {
  fn new() -> Self;

  fn stop(&mut self) {}
}
