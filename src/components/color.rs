use std::convert::Into;

///Component represents the color of an entity
#[derive(Debug, Clone, Copy)]
pub struct Color ([f32; 4]);

impl Color {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Color ([r, g, b, a])
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    self.0
  }
}