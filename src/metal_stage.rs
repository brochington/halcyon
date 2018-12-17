use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;
use winit::{ WindowBuilder, Window, EventsLoop, Event, WindowEvent, DeviceEvent };
use winit::os::macos::WindowExt;
use metal::*;
use cocoa::base::id as cocoa_id;
use cocoa::appkit::{ NSWindow, NSView };
use std::mem;
use core_graphics::geometry::CGSize;
use objc::runtime::YES;

use crate::stage::Stage;
use crate::scene_2d::{ Scene2D };
use crate::engines::Engines;
use crate::internal_actions::{ InternalActions, root_reducer };
use crate::state_management::{ Store };
use crate::internal_state::InternalState;
use crate::geometry::Geometries;
use crate::geometry::Triangle2D;

fn prepare_pipeline_state<'a>(device: &DeviceRef, library: &LibraryRef) -> RenderPipelineState {
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
}

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
  pub engine: Engines,
  pub events_loop: EventsLoop,
  pub window: Window,
  pub internal_store: Store<InternalState, InternalActions>,
}


impl Stage for MetalStage {
  fn new() -> MetalStage {

    let events_loop = EventsLoop::new();
    let init_state = InternalState{ playing: true };
    let internal_store = Store::init(root_reducer, init_state);
    
    let window = WindowBuilder::new().build(&events_loop).unwrap();

    MetalStage { 
      engine: Engines::Metal,
      events_loop: events_loop,
      window,
      internal_store
    }
  }
}

pub fn play(mut stage: MetalStage, mut cb: Box<FnMut() -> Scene2D>) {
  let mut playing = true;

  let mut i_store = stage.internal_store;

  let ns_window: cocoa_id = unsafe { mem::transmute(stage.window.get_nswindow()) };
  let device = Device::system_default(); // create a MTLDevice

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

  let draw_size = stage.window.get_inner_size().unwrap();
  layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

  let library = get_library_from_source(&device, "examples/metal/shaders.metal");
  let pipeline_state = prepare_pipeline_state(&device, &library);
  let command_queue = device.new_command_queue();

  while playing {
    let current_state = i_store.get_state();

    if !current_state.playing {
      playing = false;
    }

    stage.events_loop.poll_events(|event| {
      match event {
        Event::WindowEvent{ event: WindowEvent::CloseRequested, .. } => i_store.dispatch(InternalActions::Stop),
        Event::WindowEvent{ event: WindowEvent::CursorMoved{ position, .. }, .. } => i_store.dispatch(InternalActions::CursorMoved{ x: position.x, y: position.y }),
        Event::DeviceEvent{ event: DeviceEvent::MouseMotion{ delta, .. }, .. } => i_store.dispatch(InternalActions::CursorMoved{x: delta.0, y: delta.1 }),
        _ => ()
      }
    });

    let current_scene = cb();

    let mut tri = Triangle2D::default();
    if let Geometries::Triangle2D(triangle) = &current_scene.geometries[0] {
      tri = triangle.clone();
    };
    
    let vbuf = {
      // x, y, r, g, b
      let vertex_data = [
        tri.points[0].x as f32, tri.points[0].y as f32, 1.0, 0.0, 0.0,
        tri.points[1].x as f32, tri.points[1].y as f32, 0.0, 1.0, 0.0,
        tri.points[2].x as f32, tri.points[2].y as f32, 0.0, 0.0, 1.0,
      ];

      device.new_buffer_with_data(
        unsafe { mem::transmute(vertex_data.as_ptr()) },
        (vertex_data.len() * mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache
      )
    };

    if let Some(drawable) = layer.next_drawable() {
      // Create a Render Pass Descriptor
      let render_pass_descriptor = RenderPassDescriptor::new();
      let color_attachment = render_pass_descriptor.color_attachments().object_at(0).unwrap();
      color_attachment.set_texture(Some(drawable.texture()));
      color_attachment.set_load_action(MTLLoadAction::Clear);
      color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 0.0));
      color_attachment.set_store_action(MTLStoreAction::Store);

      // Create a Command Buffer
      let command_buffer = command_queue.new_command_buffer();

      // Create a Render Command Encoder
      let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
      encoder.set_render_pipeline_state(&pipeline_state);
      encoder.set_vertex_buffer(0, Some(&vbuf), 0);
      encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
      encoder.end_encoding();

      // Commit your Command Buffer
      command_buffer.present_drawable(&drawable);
      command_buffer.commit();
    }
  }
}

