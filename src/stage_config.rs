#[derive(Debug)]
pub struct StageConfig {}

impl StageConfig {
  pub fn new() -> StageConfig {
    StageConfig {
      ..Default::default()
    }
  }
}

impl Default for StageConfig {
  fn default() -> StageConfig {
    StageConfig {}
  }
}
