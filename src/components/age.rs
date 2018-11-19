///Component represents the age of an entity
#[derive(Default, Debug, Clone, Copy)]
pub struct Age {
  pub seconds: f32,
  pub max: Option<f32>,
}