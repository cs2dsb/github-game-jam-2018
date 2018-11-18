use amethyst::{
  assets::{
    AssetStorage,
    Loader,
    ProgressCounter,
  },
  audio::{
    output::Output,
    Source,
    SourceHandle,
    OggFormat,
  },
  utils::application_root_dir,
};

use ::config::SoundConfig;

const SOUND_PATH: &'static str = "assets/sound";
const SPAWN_FILE: &'static str = "spawn.ogg";
const EXIT_FILE: &'static str = "exit.ogg";
const LIFT_FILE: &'static str = "lift.ogg";

pub struct Sounds {
  pub volume: f32,
  spawn: SourceHandle,
  exit: SourceHandle,
  lift: SourceHandle,
}

fn load_ogg_file(loader: &Loader, storage: &AssetStorage<Source>, progress: &mut ProgressCounter, file: &str) -> SourceHandle {
  loader
    .load(
      file,
      OggFormat,
      (),
      progress,
      storage)
}

impl Sounds {
  pub fn new(loader: &Loader, storage: &AssetStorage<Source>, progress: &mut ProgressCounter, sound_config: &SoundConfig) -> Self {
    let root_dir = application_root_dir();
    Sounds {
      volume: sound_config.volume,
      spawn: load_ogg_file(loader, storage, progress, &format!("{}/{}/{}", root_dir, SOUND_PATH, SPAWN_FILE)),
      exit: load_ogg_file(loader, storage, progress, &format!("{}/{}/{}", root_dir, SOUND_PATH, EXIT_FILE)),
      lift: load_ogg_file(loader, storage, progress, &format!("{}/{}/{}", root_dir, SOUND_PATH, LIFT_FILE)),
    }
  }

  //This is called by the DJ system when the previous music track ends
  pub fn next_music(&mut self) -> Option<SourceHandle> {
    None
  }

  pub fn play_spawn(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.spawn) {
      output.play_once(sound, self.volume);
    }
  }

  pub fn play_exit(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.exit) {
      output.play_once(sound, self.volume);
    }
  }

  //Not sure how to trigger this since the lift force is all internal to the
  //physics system. Maybe swap force generator for a sensor + impulse...
  #[allow(dead_code)]
  pub fn play_lift(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.lift) {
      output.play_once(sound, self.volume);
    }
  }
}