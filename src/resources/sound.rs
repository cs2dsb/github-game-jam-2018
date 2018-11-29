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
    Mp3Format,
  },
  utils::application_root_dir,
};

use ::config::SoundConfig;

const SOUND_PATH: &'static str = "assets/sound";
const SPAWN_FILE: &'static str = "spawn.ogg";
const EXIT_FILE: &'static str = "exit.ogg";
const LIFT_FILE: &'static str = "lift.ogg";
const DEATH_FILE: &'static str = "death.mp3";
const EXODUS_FILE: &'static str = "woo.ogg";

const SPAWN_VOLUME: f32 = 0.8;
const EXIT_VOLUME: f32 = 0.6;
const LIFT_VOLUME: f32 = 0.6;
const DEATH_VOLUME: f32 = 1.0;
const EXODUS_VOLUME: f32 = 0.15;

///Resource containing the sound effects the game uses.
pub struct Sounds {
  pub volume: f32,
  spawn: SourceHandle,
  exit: SourceHandle,
  lift: SourceHandle,
  death: SourceHandle,
  exodus: SourceHandle,
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

fn load_mp3_file(loader: &Loader, storage: &AssetStorage<Source>, progress: &mut ProgressCounter, file: &str) -> SourceHandle {
  loader
    .load(
      file,
      Mp3Format,
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
      death: load_mp3_file(loader, storage, progress, &format!("{}/{}/{}", root_dir, SOUND_PATH, DEATH_FILE)),
      exodus: load_ogg_file(loader, storage, progress, &format!("{}/{}/{}", root_dir, SOUND_PATH, EXODUS_FILE)),
    }
  }

  //This is called by the DJ system when the previous music track ends
  pub fn next_music(&mut self) -> Option<SourceHandle> {
    None
  }

  pub fn play_spawn(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.spawn) {
      output.play_once(sound, self.volume * SPAWN_VOLUME);
    }
  }

  pub fn play_exit(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.exit) {
      output.play_once(sound, self.volume * EXIT_VOLUME);
    }
  }

  pub fn play_lift(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.lift) {
      output.play_once(sound, self.volume * LIFT_VOLUME);
    }
  }

  pub fn play_death(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.death) {
      output.play_once(sound, self.volume * DEATH_VOLUME);
    }
  }

  pub fn play_exodus(&self, storage: &AssetStorage<Source>, output: &Output) {
    if let Some(sound) = storage.get(&self.exodus) {
      output.play_once(sound, self.volume * EXODUS_VOLUME);
    }
  }
}