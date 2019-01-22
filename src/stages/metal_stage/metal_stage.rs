use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;
use winit::{ WindowBuilder, Window, EventsLoop, Event, WindowEvent, DeviceEvent };
use winit::os::macos::WindowExt;
use winit::dpi::LogicalSize;
use metal::*;
use cocoa::base::id as cocoa_id;
use cocoa::appkit::{ NSWindow, NSView };
use std::mem;
use core_graphics::geometry::CGSize;
use objc::runtime::YES;

use crate::stages::metal_stage::metal_stage_config::MetalStageConfig;
use crate::scene_2d::{ Scene2D };
use crate::internal_actions::{ InternalActions, root_reducer };
use crate::utils::state_management::{ Store };
use crate::utils::helpers::{ cartesian_to_clip };
use crate::utils::color::{ Color };
use crate::internal_state::InternalState;
use crate::primitive_2d::Primitives2D;
use crate::texture::{ Textures };
use crate::math::{Mat3, Vec2};

/* Compiles Metal library from shader source code path */
fn get_library_from_source(device: &Device, shader_path: &str) -> Library {
  let _options = CompileOptions::new(); // Not sure what options are.
  let program = File::open(shader_path).unwrap();
  let mut buf_reader = BufReader::new(program);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();
  device.new_library_with_source(&contents, &_options).unwrap()
}
  
#[derive(Debug)]
pub struct MetalStage {
  pub config: MetalStageConfig
}

impl MetalStage {
  pub fn new(config: MetalStageConfig) -> MetalStage {
    MetalStage { 
      config,
    }
  }
}

fn process_scene(device: &Device, scene: Scene2D) -> (Buffer, u64) {
  let prims = scene.primitives;
  let textures = scene.textures;
  let mut vertex_data: Vec<f32> = vec![];

  for prim in prims {
    match prim {
      Primitives2D::Triangle2D(triangle) => {
        let [trans_x, trans_y ] = triangle.transform.get_translation();
        let translated_points = vec![
          Vec2::new(triangle.vertices[0].x + trans_x, triangle.vertices[0].y + trans_y),
          Vec2::new(triangle.vertices[1].x + trans_x, triangle.vertices[1].y + trans_y),
          Vec2::new(triangle.vertices[2].x + trans_x, triangle.vertices[2].y + trans_y)
        ];
        let clip_points = cartesian_to_clip(1000.0, 1000.0, &translated_points);


        if let Some(texture) = textures.get(&triangle.texture_id) {
          match texture {
            Textures::ColorTexture(color_texture) => {
              clip_points
                .iter()
                .zip(color_texture.get_colors().iter())
                .for_each(|(coord, color)| {
                  let [r, g, b ] = color.get_rgb();
                  vertex_data.extend(vec![
                    coord.x as f32, 
                    coord.y as f32,
                    r as f32, 
                    g as f32, 
                    b as f32
                  ]);
                });
            },
            _ => ()
          }
        }
      },
      Primitives2D::Rectangle2D(rectangle) => {
        let [ trans_x, trans_y ] = rectangle.transform.get_translation();
        let vertices = vec![
          // first triangle
          Vec2::new(trans_x, trans_y),
          Vec2::new(trans_x + rectangle.width, trans_y),
          Vec2::new(trans_x, trans_y + rectangle.height),
          // second triangle
          Vec2::new(trans_x + rectangle.width, trans_y),
          Vec2::new(trans_x, trans_y + rectangle.height),
          Vec2::new(trans_x + rectangle.width, trans_y + rectangle.height)
        ];

        let vertices = cartesian_to_clip(1000.0, 1000.0, &vertices);

        let vertices = vertices.iter().fold(vec![], |mut acc, & c| {
          acc.extend(vec![
            c.x as f32,
            c.y as f32,
            0.0 as f32,
            1.0 as f32,
            0.0 as f32
          ]);
          acc
        });

        vertex_data.extend(vertices);
      }
    }
  };
  let buffer_length = vertex_data.len() as u64;
  let buffer = create_buffer(&device, vertex_data);

  (buffer, buffer_length)

  // println!("vertex_data {:#?}", vertex_data);
  // MUST take f32 as data, NOT f64!!
  // (device.new_buffer_with_data(
  //   unsafe { mem::transmute(vertex_data.as_ptr()) },
  //   (vertex_data.len() * mem::size_of::<f32>()) as u64,
  //   MTLResourceOptions::CPUCacheModeDefaultCache
  // ), vertex_data.len() as u64)
}

// fn create_encoders(device: &Device, library: &LibraryRef, scene: Scene2D, command_buffer: &CommandBufferRef, render_pass_descriptor: &RenderPassDescriptorRef) {
//   let prims = scene.primitives;
//   let mut tri_vbuf = vec![];
//   // let mut text_data: Vec<u32> = vec![1000, 1000, 1000, 1000];
//   let mut text_data: Vec<u32> = vec![];
  

//   for prim in prims {
//     match prim {
//       Primitives2D::Triangle2D(triangle) => {
//         let translated_points = vec![
//           Vec2::new(triangle.points[0].x + triangle.position.x, triangle.points[0].y + triangle.position.y),
//           Vec2::new(triangle.points[1].x + triangle.position.x, triangle.points[1].y + triangle.position.y),
//           Vec2::new(triangle.points[2].x + triangle.position.x, triangle.points[2].y + triangle.position.y)
//         ];

//         let clip_points = cartesian_to_clip(1000.0, 1000.0, translated_points);

//         let color = match triangle.material {
//           Materials2D::BasicMaterial2D(material) => material.color,
//         };

//         let [r, g, b] = get_rgb(color);

//         let accum_points = clip_points.iter().fold(vec![], |mut acc, &c| {
//           acc.extend(vec![
//             c.x,
//             c.y,
//             r as f32,
//             g as f32,
//             b as f32,
//             0.0,
//             0.0,
//           ]);

//           acc
//         });

//         tri_vbuf.extend(accum_points);
//       },
//       Primitives2D::Rectangle2D(rectangle) => {
//         let Vec2 { x: trans_x, y: trans_y } = rectangle.transform.get_translation();
//         let vertices = vec![
//           // first triangle
//           Vec2::new(trans_x, trans_y + rectangle.height),
//           Vec2::new(trans_x, trans_y),
//           Vec2::new(trans_x + rectangle.width, trans_y),
//           // second triangle
//           Vec2::new(trans_x + rectangle.width, trans_y),
//           Vec2::new(trans_x + rectangle.width, trans_y + rectangle.height),
//           Vec2::new(trans_x, trans_y + rectangle.height)
//         ];

//         let vertices = cartesian_to_clip(1000.0, 1000.0, vertices);

//         let verts = vec![
//           Vec2::new(0.0, 1.0),
//           Vec2::new(0.0, 0.0),
//           Vec2::new(1.0, 0.0),
//           Vec2::new(1.0, 0.0),
//           Vec2::new(1.0, 1.0),
//           Vec2::new(0.0, 1.0),
//         ];

//         let (vertices, _, _) = vertices.iter().fold((vec![], 0, verts), |acc, & c| {
//           let (mut accum, count, vert) = acc;
//           accum.extend(vec![
//             c.x,
//             c.y,
//             0.5,
//             0.2,
//             0.7,
//             vert[count].x,
//             vert[count].y
//           ]);
//           let new_count = count + 1;
//           (accum, new_count, vert)
//         });

//         tri_vbuf.extend(vertices);
//         // println!("l {:#?}", rectangle.texture.pixels.len());
//         for c in rectangle.texture.pixels {
//           let [ r, g, b ] = get_rgb(c);
//           let total = (r * 255.0) as u32;
//           let total = total.wrapping_shl(8);
//           let total = total + (g * 255.0) as u32;
//           let total = total.wrapping_shl(8);
//           let total = total + (b * 255.0) as u32;
//           // let total = total.wrapping_shl(8);
//           text_data.push(total);
//         }
//       }
//     }
//   }

//   // encode triangles
//   let pipeline_state = prepare_pipeline_state(&device, &library);
//   // println!("buf {:#?}", tri_vbuf);
//   let vbuf = device.new_buffer_with_data(
//     unsafe { mem::transmute(tri_vbuf.as_ptr()) },
//     (tri_vbuf.len() * mem::size_of::<f32>()) as u64,
//     MTLResourceOptions::CPUCacheModeDefaultCache
//   );

//   let region = MTLRegion {
//     origin: MTLOrigin {
//       x: 0,
//       y: 0,
//       z: 0
//     },
//     size: MTLSize {
//       width: 254,
//       height: 254,
//       depth: 1
//     }
//   };
//   // test making of a texture
//   let texture_descriptor = TextureDescriptor::new();
//   texture_descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
//   texture_descriptor.set_width(254);
//   texture_descriptor.set_height(254);
//   texture_descriptor.set_storage_mode(MTLStorageMode::Managed);

//   // println!("t {:#?}", text_data);
//   let my_texture = device.new_texture(&texture_descriptor);
//   my_texture.replace_region(
//     region,
//     0,
//     1016, // width of the image * 4 (to per each byte)
//     unsafe { mem::transmute(text_data.as_ptr())}
//   );

//   // println!("tri_offset! {:#?}", vbuf);
//   // Create a Render Command Encoder
//   let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
//   encoder.set_render_pipeline_state(&pipeline_state);
//   encoder.set_vertex_buffer(0, Some(&vbuf), 0);
//   encoder.set_fragment_texture(0, Some(&my_texture));
//   encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, tri_vbuf.len() as u64 / 7);
  
//   encoder.end_encoding();
// }

fn create_internal_store(width: f64, height: f64) -> Store<InternalState, InternalActions> {
  let init_state = InternalState{ 
    playing: true,
    width,
    height
  };

  Store::init(root_reducer, init_state)
}

fn create_events_loop() -> EventsLoop {
  EventsLoop::new()
}

fn create_window(width: f64, height: f64, events_loop: &EventsLoop) -> (winit::Window, *mut objc::runtime::Object) {
  let window = WindowBuilder::new()
    .with_dimensions(LogicalSize { width: width as f64, height: height as f64 })
    .build(&events_loop)
    .unwrap(); 

  let ns_window: cocoa_id = unsafe { mem::transmute(window.get_nswindow()) };

  (window, ns_window)
}

fn create_device() -> Device {
  Device::system_default()
}

fn create_and_init_layer(window: &Window, ns_window: *mut objc::runtime::Object, device: &Device) -> CoreAnimationLayer {
  // Create CoreAnimationLayer, which provides the "drawable", 
  // which I assume is kinda like a GL context.
  let layer = CoreAnimationLayer::new();
  layer.set_device(&device);
  layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm); // 8 bytes for Blue, Green, Red and Alpha, in that order — with normalized values between 0 and 1.
  layer.set_presents_with_transaction(false);

  unsafe {
    let view = ns_window.contentView();
    view.setWantsBestResolutionOpenGLSurface_(YES);
    view.setWantsLayer(YES);
    view.setLayer(mem::transmute(layer.as_ref()));
  }

  let draw_size = &window.get_inner_size().unwrap();
  layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

  layer
}

fn create_and_init_render_pass_descriptor(texture: &TextureRef) -> &'static RenderPassDescriptorRef {
  // Create a Render Pass Descriptor
  let render_pass_descriptor = RenderPassDescriptor::new();
  let color_attachment = render_pass_descriptor.color_attachments().object_at(0).unwrap();
  color_attachment.set_texture(Some(texture));
  color_attachment.set_load_action(MTLLoadAction::Clear);
  color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 0.0));
  color_attachment.set_store_action(MTLStoreAction::Store);

  render_pass_descriptor
}

fn create_buffer(device: &Device, vertex_data: Vec<f32>) -> Buffer {
  device.new_buffer_with_data(
    unsafe { mem::transmute(vertex_data.as_ptr()) },
    (vertex_data.len() * mem::size_of::<f32>()) as u64,
    MTLResourceOptions::CPUCacheModeDefaultCache
  )
}

// TODO: update buffer goes here

fn translate_vertices(transform: Mat3, vertices: &Vec<Vec2>) -> Vec<Vec2> {
  let [ trans_x, trans_y ] = transform.get_translation();

  vertices
    .iter()
    .map(| Vec2 {x, y } | {
      Vec2::new(x + trans_x, y + trans_y)
    })
    .collect::<Vec<Vec2>>()
}

fn get_rect_vertices(transform: Mat3, width: f64, height: f64) -> Vec<Vec2> {
  let [ trans_x, trans_y ] = transform.get_translation();

  vec![
    Vec2::new(trans_x, trans_y + height),
    Vec2::new(trans_x, trans_y),
    Vec2::new(trans_x + width, trans_y),
    Vec2::new(trans_x + width, trans_y),
    Vec2::new(trans_x + width, trans_y + height),
    Vec2::new(trans_x, trans_y + height),
  ]
}

// vertext = x | y | z | mode | r | g | b | a | texCoordX | texCoordY
fn get_vertex_data_from_scene(scene: &Scene2D, window_width: f64, window_height: f64) -> Vec<f32> {
  let primitives = &scene.primitives;
  let textures = &scene.textures;
  let mut vertex_data = vec![];

  for primitive in primitives {
    match primitive {
      Primitives2D::Triangle2D(triangle) => {
        let translated_vertices = translate_vertices(triangle.transform, &triangle.vertices);
        let vertices_in_clip_space = cartesian_to_clip(window_width, window_height, &translated_vertices);

        if let Some(texture) = textures.get(&triangle.texture_id) {
          match texture {
            Textures::ColorTexture(color_texture) => {
              vertices_in_clip_space
                .iter()
                .zip(color_texture.get_colors().iter())
                .for_each(|(coord, color)| {
                  let [r, g, b] = color.get_rgb();
                  vertex_data.extend(vec![
                    coord.x as f32, // x
                    coord.y as f32, // y
                    0.0, // z, not currently used
                    0.0,  // mode "0" is rgba
                    r as f32,
                    g as f32,
                    b as f32,
                    1.0, // alpha
                    0.0, // texCoordX, 
                    0.0 // texCoordY
                  ])
                })
            },
            _ => ()
          }
        }
      },
      Primitives2D::Rectangle2D(rectangle) => {
        let translated_vertices = get_rect_vertices(rectangle.transform, rectangle.width, rectangle.height);
        let vertices_in_clip_space = cartesian_to_clip(window_width, window_height, &translated_vertices);
        // println!("v {:#?}", vertices_in_clip_space);
        if let Some(texture) = textures.get(&rectangle.texture_id) {
          match texture { 
            Textures::ColorTexture(color_texture) => {
              vertices_in_clip_space
                .iter()
                .zip(color_texture.get_colors().iter())
                .for_each(|(coord, color)| {
                  // println!("color {:#?}", color);
                  let [r, g, b] = color.get_rgb();
                  vertex_data.extend(vec![
                    coord.x as f32, // x
                    coord.y as f32, // y
                    0.0, // z, not currently used
                    0.0,  // mode "0" is rgba
                    r as f32,
                    g as f32,
                    b as f32,
                    1.0, // alpha
                    0.0, // texCoordX, 
                    0.0 // texCoordY
                  ])
                }
              )
            },
            Textures::ImageTexture(image_texture) => {
              vertices_in_clip_space
                .iter()
                .zip(rectangle.uv_map.iter())
                .for_each(|(coord, uv)| {
                  vertex_data.extend(vec![
                    coord.x as f32,
                    coord.y as f32,
                    0.0, // z, not currently used
                    1.0, // mode "1" is uv texture mapping
                    0.0, // r, not used
                    0.0, // g, not used
                    0.0, // b, not used
                    1.0, // alpha, not used
                    uv.x as f32,
                    uv.y as f32
                  ])
                })

            },
            _ => ()
          }
        }
      }
    }
  }

  vertex_data
}

pub fn play(stage: MetalStage, mut cb: Box<FnMut() -> Scene2D>) {
  let MetalStageConfig { width, height } = stage.config;

    // create store
  let mut i_store = create_internal_store(width, height);

  // create events loop
  let mut events_loop = create_events_loop();

  // create device
  let device = create_device();

  // create window
  let (window, ns_window) = create_window(width as f64, height as f64, &events_loop);

  // create layer
  let layer = create_and_init_layer(&window, ns_window, &device);

  // fetch shader library files, and compile them.
  let library = get_library_from_source(&device, "examples/metal_primitives/shaders.metal");

  // create initial pipeline state. 
  // Might want to refactor shader function fetching here.
  let pipeline_state = {
    let vert = library.get_function("triangle_vertex", None).unwrap();
    let frag = library.get_function("triangle_fragment", None).unwrap();

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));
    pipeline_state_descriptor
      .color_attachments()
      .object_at(0)
      .unwrap()
      .set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    device.new_render_pipeline_state(&pipeline_state_descriptor).unwrap()
  };

  let command_queue = device.new_command_queue();

  let mut playing = true;

  while playing {
    let current_state = i_store.get_state();
    let current_playing = current_state.playing;
    let current_width = current_state.width;
    let current_height = current_state.height;

    if !current_playing {
      playing = false;
    }

    events_loop.poll_events(|event| {
      match event {
        Event::WindowEvent{ event: WindowEvent::CloseRequested, .. } => i_store.dispatch(InternalActions::Stop),
        Event::WindowEvent{ event: WindowEvent::Resized(new_size), .. } => i_store.dispatch(InternalActions::ResizeWindow { width: new_size.width, height: new_size.height }),
        Event::WindowEvent{ event: WindowEvent::CursorMoved{ position, .. }, .. } => i_store.dispatch(InternalActions::CursorMoved{ x: position.x, y: position.y }),
        Event::DeviceEvent{ event: DeviceEvent::MouseMotion{ delta, .. }, .. } => i_store.dispatch(InternalActions::CursorMoved{x: delta.0, y: delta.1 }),
        _ => ()
      }
    });

    let current_scene = cb();

    // handle textures
    
    
    // let (vertex_data, len) = process_scene(&device, current_scene);
    let vertex_data = get_vertex_data_from_scene(&current_scene, current_width, current_height);
    let vertex_data_length = vertex_data.len() as u64;
    let vertex_buffer = create_buffer(&device, vertex_data);

    if let Some(drawable) = layer.next_drawable() {
      // create render pass descriptor
      let render_pass_descriptor = create_and_init_render_pass_descriptor(drawable.texture());

      // Create a Command Buffer
      let command_buffer = command_queue.new_command_buffer();

      // Create a Render Command Encoder
      let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
      encoder.set_render_pipeline_state(&pipeline_state);
      encoder.set_vertex_buffer(0, Some(&vertex_buffer), 0);
      // TODO: maybe update this to use drawIndexedPrimitives to save some memory.
      encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, vertex_data_length / 10);
      encoder.end_encoding();

      // create_encoders(&device, &library, current_scene, &command_buffer, &render_pass_descriptor);

      // Commit your Command Buffer
      command_buffer.present_drawable(&drawable);
      command_buffer.commit();
    }
  }
}

