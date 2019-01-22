extern crate halcyon;
extern crate image;

use image::*;

use crate::halcyon::{ Scene2D, texture, stages, primitive_2d, utils };
use crate::utils::color::{ Color, ColorU8 };
use crate::primitive_2d::{ Triangle2D, Rectangle2D };
use crate::texture::{ ColorTexture, ImageTexture };
use crate::stages::metal_stage::{ MetalStage, MetalStageConfig, play};

fn main() {
  let img = image::open("examples/metal/colors_128.png").unwrap();
  let color_data = img
    .pixels()
    .map(|(_w, _h, p)| {
      ColorU8::from_rgba(
        p.data[0],
        p.data[1],
        p.data[2],
        p.data[3],
      )
    })
    .collect::<Vec<ColorU8>>();

  println!("{:#?}", color_data[0]);
  let stage_config = MetalStageConfig::new();
  let stage = MetalStage::new(stage_config);

  let mut scene = Scene2D::new();

  let basic_texture = ColorTexture::new();
  let basic_texture_id = basic_texture.get_id().clone();
  let mut image_texture = ImageTexture::new();
  image_texture.set_pixels(color_data);

  let mut tri = Triangle2D::new();
  tri.set_vertices(0.0, 0.0, 100.0, 100.0, 0.0, 100.0);
  tri.set_translation(0.0, 0.0);
  tri.set_texture_id(basic_texture_id.clone());

  scene.add_texture(ColorTexture::wrap(basic_texture));
  scene.add_primitive(Triangle2D::wrap(tri));

  let mut rect = Rectangle2D::new();
  rect.set_dimensions(100.0, 100.0);
  rect.set_translation(110.0, 0.0);
  rect.set_texture_id(image_texture.get_id());

  scene.add_primitive(Rectangle2D::wrap(rect));

  // Is there a way to not have this have to clone the scene?
  let on_update = move || {
    let next_scene = scene.clone();
    next_scene
  };

  play(stage, Box::new(on_update));
}