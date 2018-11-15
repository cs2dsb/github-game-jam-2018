use amethyst::{
  assets::{
    Prefab,
    Handle,
  },
  core::Time,
  ecs::prelude::*,
  prelude::*,
  input::is_key_down,
  renderer::{
    PosNormTex,
  },
  ui::{
    UiFinder,
    UiPrefab,
    UiText,
  },
  utils::{
    fps_counter::FPSCounter,
    scene::BasicScenePrefab,
  },
  winit::VirtualKeyCode,
};

use ::{
  config::{
    SpawnerConfig,
    LevelsConfig,
  },
  resources::SpawnStats,
  components::{
    Spawner,
  },
  levels::*,
};

const UI_UPDATE_FRAMES: u64 = 20; //How many frames to wait between ui updates

pub type RunningPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

pub struct RunningState {
  running_ui_handle: Handle<UiPrefab>,
  running_prefab_handle: Handle<Prefab<RunningPrefabData>>,
  fps_display: Option<Entity>,
  spawned_display: Option<Entity>,
  rate_display: Option<Entity>,
  killed_display: Option<Entity>,
  saved_display: Option<Entity>,
  level: Option<Level>,
}

impl<'a, 'b> SimpleState<'a, 'b> for RunningState {
  fn on_start(&mut self, data: StateData<GameData>) {
    info!("RunningState.on_start");
    let world = data.world;

    self.initialise_prefab(world);
    self.initialise_ui(world);
    self.initialise_level(world);
  }
  fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
    match &event {
      StateEvent::Window(event) => {
        if is_key_down(&event, VirtualKeyCode::Escape) {
          Trans::Quit
        } else {
          Trans::None
        }
      },
      _ => Trans::None,
    }
  }
  fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
    let world = &mut data.world;

    //Fetch the entities for the ui fields
    if self.fps_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("fps") {
          self.fps_display = Some(entity);
        }
      });
    }

    if self.spawned_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("spawned") {
          self.spawned_display = Some(entity);
        }
      });
    }

    if self.rate_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("rate") {
          self.rate_display = Some(entity);
        }
      });
    }

    if self.killed_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("killed") {
          self.killed_display = Some(entity);
        }
      });
    }

    if self.saved_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("saved") {
          self.saved_display = Some(entity);
        }
      });
    }

    //Update the ui values
    if world.read_resource::<Time>().frame_number() % UI_UPDATE_FRAMES == 0 {
      if self.fps_display.is_some() {
        let mut ui_text = world.write_storage::<UiText>();
        if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
          let fps = world.read_resource::<FPSCounter>().sampled_fps();
          fps_display.text = format!("FPS: {:.*}", 1, fps);
        }
      }

      if self.spawned_display.is_some() || self.rate_display.is_some() {
        let mut rate = None;
        for s in world.read_storage::<Spawner>().join() {
          if let Some(rate) = rate {
            assert_eq!(rate, s.frequency);
          } else {
            rate = Some(s.frequency);
          }
        }

        let spawn_stats = world.read_resource::<SpawnStats>();
        let mut ui_text = world.write_storage::<UiText>();
        if let Some(spawned_display) = self.spawned_display.and_then(|entity| ui_text.get_mut(entity)) {
          spawned_display.text = format!("SPAWN: {}/{}", spawn_stats.spawned, spawn_stats.total);
        }
        if let Some(killed_display) = self.killed_display.and_then(|entity| ui_text.get_mut(entity)) {
          killed_display.text = format!("KILLED: {}", spawn_stats.killed);
        }
        if let Some(saved_display) = self.saved_display.and_then(|entity| ui_text.get_mut(entity)) {
          saved_display.text = format!("SAVED: {}", spawn_stats.saved);
        }
        if let (Some(rate), Some(rate_display)) = (rate, self.rate_display.and_then(|entity| ui_text.get_mut(entity))) {
          let spawner_config = world.read_resource::<SpawnerConfig>();
          let rate = (rate - spawner_config.frequency_min) / (spawner_config.frequency_max - spawner_config.frequency_min);
          let rate = 1.0 - rate;
          let rate = 100.0 * rate;
          let rate = rate.round() as u32;
          rate_display.text = format!("RATE: {}", rate);
        }
      }
    }

    Trans::None
  }
}

impl RunningState {
  pub fn new(running_ui_handle: Handle<UiPrefab>, running_prefab_handle: Handle<Prefab<RunningPrefabData>>) -> Self {
    Self {
      running_ui_handle,
      running_prefab_handle,
      fps_display: None,
      spawned_display: None,
      rate_display: None,
      killed_display: None,
      saved_display: None,
      level: None,
    }
  }

  fn initialise_prefab(&self, world: &mut World) {
    world
      .create_entity()
      .with(self.running_prefab_handle.clone())
      .build();
  }

  fn initialise_ui(&self, world: &mut World) {
    world
      .create_entity()
      .with(self.running_ui_handle.clone())
      .build();
  }

  fn initialise_level<'a>(&mut self, world: &mut World) {
    let level = Level::new(&world.read_resource::<LevelsConfig>());
    level.load(world);
    self.level = Some(level);
  }
}