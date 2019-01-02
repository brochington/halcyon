extern crate halcyon;
extern crate winit;
extern crate rand;
extern crate image;

pub mod app_state;
pub mod actions;

use crate::halcyon::{ Stage,  MetalStage, metal_stage, Scene2D, primitive_2d, material_2d, texture, state_management, scene_2d, math, utils };
use crate::primitive_2d::{ Primitives2D, Triangle2D, Rectangle2D };
// use crate::primitive_2d::Primitives2D;
// use crate::primitive_2d::{ Primitive2D, Primitives2D };
use crate::texture::{ ImageTexture };
use crate::material_2d::Materials2D;
use crate::utils::color::{ Color, ColorModels, get_rgb };
use crate::state_management::Store;
use crate::actions::{ Actions, root_reducer };
use crate::metal_stage::play;
use crate::math::Vec2;

use image::*;

fn main() {
  // let img = image::open("examples/metal/colors_128.png").unwrap();
  // println!("color type? {:#?}", img.color());

  // let stage = MetalStage::new();
  // let mut beginning_scene = Scene2D::new();

  // let tri1 = Triangle2D::new();
  // let tri1 = Triangle2D::set_points(tri1, vec![
  //   Vec2::new(0.0, 0.0),
  //   Vec2::new(100.0, 100.0),
  //   Vec2::new(0.0, 100.0)
  // ]);
  // let tri1 = Triangle2D::set_position(0.0, 0.0, tri1);
  // let tri1 = Triangle2D::set_basic_color(tri1, 1.0, 0.0, 0.0);
  // beginning_scene.add(Triangle2D::wrap(tri1));

  // let tri2 = Triangle2D::new();
  // let tri2 = Triangle2D::set_points(tri2, vec![
  //   Vec2::new(10.0, 00.0),
  //   Vec2::new(110.0, 00.0),
  //   Vec2::new(110.0, 100.0)
  // ]);
  // let tri2 = Triangle2D::set_position(0.0, 0.0, tri2);
  // beginning_scene.add(Triangle2D::wrap(tri2));

  // let tri3 = Triangle2D::new();
  // let tri3 = Triangle2D::set_points(tri3, vec![
  //   Vec2::new(120.0, 0.0),
  //   Vec2::new(220.0, 100.0),
  //   Vec2::new(120.0, 100.0)
  // ]);
  // let tri3 = Triangle2D::set_position(0.0, 0.0, tri3);
  // beginning_scene.add(Triangle2D::wrap(tri3));

  // let mut texture = ImageTexture::new();

  // let color_data = img
  //   .pixels()
  //   .map(|(_w, _h, pixel)| {
  //     Color {
  //       model: ColorModels::RGBA,
  //       color: vec![
  //         (pixel.data[0] as f32) / 255.0,
  //         (pixel.data[1] as f32) / 255.0,
  //         (pixel.data[2] as f32) / 255.0,
  //         (pixel.data[3] as f32) / 255.0 
  //       ]
  //     }
  //   }).collect::<Vec<Color>>();
  // // println!("color_data {:#?}", color_data.len());
  // texture.pixels = color_data;
  // let ( width, height ) = img.dimensions();
  // // println!("ss {:#?} {:#?}", width, height);
  // texture.width = width;
  // texture.height = height;

  // let rect1 = Rectangle2D::new();
  // let rect1 = Rectangle2D::set_texture(rect1, texture);
  // let rect1 = Rectangle2D::set_dimensions(rect1, 127.0, 127.0);
  // let rect1 = Rectangle2D::translate(rect1, 400.0, 400.0);
  // beginning_scene.add(Primitives2D::Rectangle2D(rect1));

  // let initial_state = app_state::AppState {
  //   scenes: vec![beginning_scene]
  // };

  // let mut store = Store::init(root_reducer, initial_state);

  // let on_update = move || {
  //   let current_state = store.get_state();
  //   let mut next_scene = current_state.scenes[0].clone();

  //   next_scene.primitives = next_scene.primitives
  //     .into_iter()
  //     .map(|p| {
  //       let updated_prim = match p {
  //         Primitives2D::Triangle2D(triangle) => {
  //           let color = match &triangle.material {
  //             Materials2D::BasicMaterial2D(material) => material.color.clone(),
  //           };

  //           let [r, g, b ] = get_rgb(color);

  //           let tri = Triangle2D::set_basic_color(
  //             triangle, 
  //             if r >= 1.0 { rand::random() } else { r + 0.01 }, 
  //             if g >= 1.0 { rand::random() } else { g + 0.01 }, 
  //             if b >= 1.0 { rand::random() } else { b + 0.01 }, 
  //           );

  //           Primitives2D::Triangle2D(tri)
  //         },
  //         Primitives2D::Rectangle2D(rect) => Primitives2D::Rectangle2D(rect)
  //       };

  //       updated_prim
  //     })
  //     .collect::<Vec<Primitives2D>>();

  //   store.dispatch(Actions::SetScene(next_scene.clone()));

  //   next_scene
  // };

  // play(stage, Box::new(on_update));
}