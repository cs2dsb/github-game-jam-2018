use amethyst::core::cgmath::Vector2;

pub struct SpawnerParams {
  pub spawn_size: Vector2<f32>,
  pub spawn_max: u32,
  pub frequency: f32,
}

#[derive(Debug, Clone)]
pub struct Spawner {
  pub spawn_size: Vector2<f32>,
  pub spawn_max: u32,
  pub frequency: f32,
  pub spawn_count: u32,
  pub elapsed: f32,
  _private: (),
}

impl Spawner {
  pub fn new(params: SpawnerParams) -> Self {
    Self {
      spawn_size: params.spawn_size,
      spawn_max: params.spawn_max,
      frequency: params.frequency,
      spawn_count: 0,
      elapsed: 0.0,
      _private: (),
    }
  }
}