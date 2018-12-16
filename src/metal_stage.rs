use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use winit::{ WindowBuilder, Window, EventsLoop, Event, WindowEvent, DeviceEvent };
use winit::os::macos::WindowExt;
use metal::*;
use cocoa::base::id as cocoa_id;
use cocoa::appkit::{ NSWindow, NSView };
use std::mem;
use core_graphics::geometry::CGSize;
use objc::runtime::YES;

use crate::stage::Stage;
use crate::scene_2d::Scene2D;
use crate::enums::Engines;
use crate::internal_actions::{ InternalActions, root_reducer };
use crate::state_management::{ Store };
use crate::internal_state::InternalState;

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

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
  //descriptor.color_attachments().set_object_at(0, MTLRenderPassColorAttachmentDescriptor::alloc());
  //let color_attachment: MTLRenderPassColorAttachmentDescriptor = unsafe { msg_send![descriptor.color_attachments().0, _descriptorAtIndex:0] };//descriptor.color_attachments().object_at(0);
  let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

  color_attachment.set_texture(Some(texture));
  color_attachment.set_load_action(MTLLoadAction::Clear);
  color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.0, 0.7, 1.0)); // sets the red color
  color_attachment.set_store_action(MTLStoreAction::Store);
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
  pub scene_2d: Option<Scene2D>,
  pub events_loop: EventsLoop,
  pub window: Window,
  // pub internal_store: Store<InternalState, InternalActions>,
}

impl MetalStage {
  pub fn play(&mut self) {
  // pub fn play<S: Clone, A>(&mut self, mut store: Store<S, A>) {
    let mut playing = true;
    
    let init_state = InternalState{ playing: true };
    let mut internal_store = Store::init(root_reducer, init_state);

    while playing {
      let current_state = internal_store.get_state();

      if !current_state.playing {
        playing = false;
      }

      self.events_loop.poll_events(|event| {
        match event {
          Event::WindowEvent{ event: WindowEvent::CloseRequested, .. } => internal_store.dispatch(InternalActions::Stop),
          Event::WindowEvent{ event: WindowEvent::CursorMoved{ position, .. }, .. } => internal_store.dispatch(InternalActions::CursorMoved{ x: position.x, y: position.y }),
          Event::DeviceEvent{ event: DeviceEvent::MouseMotion{ delta, ..}, .. } => internal_store.dispatch(InternalActions::CursorMoved{x: delta.0, y: delta.1 }),
          _ => ()
        }
      });
    }
  }
}

impl Stage for MetalStage {
  fn new() -> MetalStage {

    let mut events_loop = EventsLoop::new();
    
    let window = WindowBuilder::new().build(&events_loop).unwrap();
    let ns_window: cocoa_id = unsafe { mem::transmute(window.get_nswindow()) };
    let device = Device::system_default(); // create a MTLDevice

    // Create CoreAnimationLayer, which provides the "drawable", 
    // which is assume is kinda like a GL context.
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

    let draw_size = window.get_inner_size().unwrap();
    layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

    let library = get_library_from_source(&device, "examples/metal/shaders.metal");
    let pipeline_state = prepare_pipeline_state(&device, &library);
    let command_queue = device.new_command_queue();

    MetalStage { 
      engine: Engines::Metal,
      scene_2d: None,
      events_loop: events_loop,
      window,
      // internal_store
    }
  }

  fn add_scene_2d(&mut self, scene: Scene2D) {
    self.scene_2d = Some(scene);
  }
}
