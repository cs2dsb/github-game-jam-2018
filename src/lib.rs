#![feature(custom_attribute)]

extern crate amethyst;
extern crate fern;
extern crate chrono;
extern crate rand;

extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;

extern crate random_color;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use log::LevelFilter;
use amethyst::{
  prelude::*,
  renderer::{
    DisplayConfig,
  },
  utils::{
    application_root_dir,
  },
  input::InputBundle,
  //audio::AudioBundle,
  assets::PrefabLoaderSystem,
};

mod config;
use config::load_game_config;

mod systems;
use systems::register_systems;

mod rendering;
use rendering::configure_rendering;

mod states;
use states::{
  LoadingState,
  RunningPrefabData,
};

mod components;
mod resources;
mod levels;

fn create_logger(level: LevelFilter) {
  use std::io;

  let gfx_device_gl_level = if level > LevelFilter::Warn {
    LevelFilter::Warn
  } else {
     level
  };

  let color_config = fern::colors::ColoredLevelConfig::new();
  fern::Dispatch::new()
    .chain(io::stdout())
    .level(level)
    .level_for("gfx_device_gl", gfx_device_gl_level)
    .format(move |out, message, record| {
      let color = color_config.get_color(&record.level());
      out.finish(format_args!(
        "{time}: [{level}][{target}] {color}{message}{color_reset}",
        time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        level = record.level(),
        target = record.target(),
        color = format!("\x1B[{}m", color.to_fg_str()),
        message = message,
        color_reset = "\x1B[0m",
      ))
    })
    .apply()
    .expect("Failed to create fern logger");
}

pub fn run() -> Result<(), amethyst::Error> {

  let app_root = application_root_dir();
  let assets_path = format!("{}/assets/", app_root);
  let binding_path = format!("{}/resources/bindings_config.ron", app_root);

  let game_config = load_game_config().expect("GameConfig failed to load");

  //Custom create log to silence "Created buffer" spam every frame
  create_logger(game_config.log_level);

  let display_config = DisplayConfig::load(&format!("{}/resources/display_config.ron", app_root));

  //TODO: Clean up this mess. The configure_rendering and register_systems functions are really fragile
  let game_data = GameDataBuilder::default()
    .with_bundle(InputBundle::<String, String>::new()
      .with_bindings_from_file(&binding_path)?)?;

  let game_data = configure_rendering(
    register_systems(game_data)?, display_config)?
    //.with_bundle(fly_control_bundle)?
    .with(PrefabLoaderSystem::<RunningPrefabData>::default(), "", &[]);

    //.with_bundle(AudioBundle::new(
      //|music: &mut audio::Music| music.music.next() //Music in a loop
    //  |_music: &mut audio::Music| None //No music
    //))?

  let mut game = Application::build(assets_path, LoadingState::default())?
    .with_resource(game_config.pawn)
    .with_resource(game_config.physics)
    .with_resource(game_config.camera)
    .with_resource(game_config.spawner)
    .with_resource(game_config.levels)
    .build(game_data)?;
  game.run();
  Ok(())
}
