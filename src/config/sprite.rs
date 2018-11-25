#[derive(Debug, Deserialize, Serialize)]
pub struct SpriteConfig {
  pub name: String,
  //These are the raw sizes in the spritesheet
  pub sheet_x: u32,
  pub sheet_y: u32,
  pub sheet_height: u32,
  pub sheet_width: u32,
  //This is the output size and offsets
  pub scaled_x: u32,
  pub scaled_y: u32,
  pub scaled_width: u32,
  pub scaled_height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpritesConfig {
  pub sheet_width: u32,
  pub sheet_height: u32,
  pub sprites: Vec<SpriteConfig>,
}

impl Default for SpritesConfig {
  fn default() -> Self {
    Self {
      sheet_width: 0,
      sheet_height: 0,
      sprites: Vec::new(),
    }
  }
}