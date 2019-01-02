extern crate halcyon;

use crate::halcyon::{ Scene2D, texture, stages, primitive_2d };
use crate::primitive_2d::{ Triangle2D, Rectangle2D };
use crate::texture::ColorTexture;
use crate::stages::metal_stage::{ MetalStage, MetalStageConfig, play};

fn main() {
  let stage_config = MetalStageConfig::new();
  let stage = MetalStage::new(stage_config);

  let mut scene = Scene2D::new();

  let basic_texture = ColorTexture::new();

  let mut tri = Triangle2D::new();
  tri.set_vertices(0.0, 0.0, 100.0, 100.0, 0.0, 100.0);
  tri.set_translation(0.0, 0.0);
  tri.set_texture_id(basic_texture.get_id());

  scene.add_texture(ColorTexture::wrap(basic_texture));
  scene.add_primitive(Triangle2D::wrap(tri));

  let mut rect = Rectangle2D::new();
  rect.set_dimensions(100.0, 100.0);
  rect.set_translation(110.0, 0.0);

  scene.add_primitive(Rectangle2D::wrap(rect));

  // Is there a way to not have this have to clone the scene?
  let on_update = move || {
    let next_scene = scene.clone();
    next_scene
  };

  play(stage, Box::new(on_update));
}