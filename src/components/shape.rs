use amethyst::{
  renderer::Shape as aShape,
};

///Component represents the shape of an entity
#[derive(Debug, Clone)]
pub struct Shape {
  pub shape: aShape,
  pub scale: (f32, f32, f32),
}