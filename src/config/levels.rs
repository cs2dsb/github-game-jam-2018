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
pub struct LevelConfig {
  pub name: Option<String>,
  pub description: Option<String>,
  pub walls: Option<CuboidSet>,
  pub deadly_areas: Option<CuboidSet>,
  pub exits: Option<CuboidSet>,
  pub spawners: Option<CuboidSet>,
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