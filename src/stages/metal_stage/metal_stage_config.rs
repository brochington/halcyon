#[derive(Debug)]
pub struct MetalStageConfig {
  pub height: f64,
  pub width: f64,
}

impl MetalStageConfig {
  pub fn new() -> MetalStageConfig {
    MetalStageConfig {
      ..Default::default()
    }
  }
}

impl Default for MetalStageConfig {
  fn default() -> MetalStageConfig {
    MetalStageConfig {
      width: 1000.0,
      height: 1000.0
    }
  }
}
