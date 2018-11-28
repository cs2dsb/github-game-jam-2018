use amethyst::core::cgmath::Vector3;

use ::components::Color;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cuboid {
  pub size: Vector3<f32>,
  pub position: Vector3<f32>,
  pub color: Option<Color>,
  pub rotation: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CuboidSet {
  pub list: Vec<Cuboid>,
  pub color: Option<Color>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CameraOverrides {
  pub offset: Option<Vector3<f32>>,
  pub convergence_speed: Option<f32>,
  pub position: Option<Vector3<f32>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpawnOverides {
  pub freq: f32,
  pub max: u32,
  pub win_ratio: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LevelConfig {
  pub name: Option<String>,
  pub description: Option<String>,
  pub walls: Option<CuboidSet>,
  pub deadly_areas: Option<CuboidSet>,
  pub exits: Option<CuboidSet>,
  pub spawners: Option<CuboidSet>,
  pub blocks: Option<CuboidSet>,
  pub spawn_overrides: Option<SpawnOverides>,
  pub camera_overrides: Option<CameraOverrides>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LevelsConfig {
  pub start_level: Option<usize>,
  pub levels: Vec<LevelConfig>,
}

impl Default for LevelsConfig {
  fn default() -> Self {
    Self {
      start_level: None,
      levels: Vec::new(),
    }
  }
}