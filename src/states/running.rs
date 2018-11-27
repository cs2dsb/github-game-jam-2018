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
    Hidden,
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
  },
  resources::{
    SpawnStats,
    Level,
  },
  components::{
    Spawner,
  },
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
  name_display: Option<Entity>,
  description_display: Option<Entity>,
}

impl<'a, 'b> SimpleState<'a, 'b> for RunningState {
  fn on_start(&mut self, data: StateData<GameData>) {
    info!("RunningState.on_start");
    let world = data.world;

    self.initialise_prefab(world);
    self.initialise_ui(world);
  }
  fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
    match &event {
      StateEvent::Window(event) => {
        if is_key_down(&event, VirtualKeyCode::Escape) {
          return Trans::Quit;
        }
      },
      _ => {},
    }
    Trans::None
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

    if self.name_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("name") {
          self.name_display = Some(entity);
        }
      });
    }

    if self.description_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("description") {
          self.description_display = Some(entity);
        }
      });
    }

    /*
    if self.level_text_update_needed {
      let mut ui_text = world.write_storage::<UiText>();
      if let (Some(level), Some(name_display), Some(description_display)) =
        (&self.level, &self.name_display, &self.description_display)
      {
        self.level_text_update_needed = false;
        if let Some(name_display) = ui_text.get_mut(*name_display) {
          name_display.text = level.name();
        }
        if let Some(description_display) = ui_text.get_mut(*description_display) {
          description_display.text = level.description();
        }
      }
    }
    */


    //Update the ui values
    if world.read_resource::<Time>().frame_number() % UI_UPDATE_FRAMES == 0 {
      if let (Some(name_display), Some(description_display)) =
        (self.name_display, self.description_display)
      {
        let level = world.read_resource::<Level>();
        let mut hidden = world.write_storage::<Hidden>();
        let mut ui_text = world.write_storage::<UiText>();

        if level.runtime < 5.0 {
          if hidden.contains(name_display) {
            hidden.remove(name_display);
          }
          if hidden.contains(description_display) {
            hidden.remove(description_display);
          }

          if let Some(name_display) = ui_text.get_mut(name_display) {
            if name_display.text.is_empty() {
              if let Some(ref name) = level.levels[level.current_level].name {
                name_display.text.push_str(name);
              }
            }
          }

          if let Some(description_display) = ui_text.get_mut(description_display) {
            if description_display.text.is_empty() {
              if let Some(ref description) = level.levels[level.current_level].description {
                description_display.text.push_str(description);
              }
            }
          }
        } else {
          if !hidden.contains(name_display) {
            hidden
              .insert(name_display, Hidden)
              .expect("Failed to insert component");

            if let Some(name_display) = ui_text.get_mut(name_display) {
              name_display.text.clear();
            }
          }
          if !hidden.contains(description_display) {
            hidden
              .insert(description_display, Hidden)
              .expect("Failed to insert component");

            if let Some(description_display) = ui_text.get_mut(description_display) {
              description_display.text.clear();
            }
          }
        }
      }

      let mut ui_text = world.write_storage::<UiText>();

      if self.fps_display.is_some() {
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
      name_display: None,
      description_display: None,
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
}