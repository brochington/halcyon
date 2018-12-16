// extern crate winit;
// extern crate metal;
// extern crate cocoa;
// extern crate core_graphics;
// #[macro_use] extern crate objc;

pub mod stage_config;
pub mod stage;
pub mod metal_stage;
pub mod enums;
pub mod math;

pub mod geometry;
pub mod scene_2d;
pub mod state_management;
pub mod internal_actions;
pub mod internal_state;


pub use crate::stage_config::StageConfig;
pub use crate::stage::*;
pub use crate::metal_stage::*;
pub use crate::scene_2d::*;