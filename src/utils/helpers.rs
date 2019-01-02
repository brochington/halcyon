use crate::math::{ Vec2 };

pub fn cartesian_to_clip(world_width: f64, world_height: f64, points: &Vec<Vec2>) -> Vec<Vec2> {
  points
    .into_iter()
    .map(|p| Vec2::new(
      ((p.x * 2.0) / world_width) - 1.0, 
      (((p.y * 2.0) / world_height) - 1.0) * -1.0 // flip Y axis
    ))
    .collect::<Vec<Vec2>>()
}