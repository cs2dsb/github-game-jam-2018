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
  resources::{
    SpawnStats,
    Level,
  },
};

const UI_UPDATE_FRAMES: u64 = 20; //How many frames to wait between ui updates

pub type RunningPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

///Main state of the game. Main work done here is updating the gui.
pub struct RunningState {
  running_ui_handle: Handle<UiPrefab>,
  running_prefab_handle: Handle<Prefab<RunningPrefabData>>,
  fps_display: Option<Entity>,

  spawned_display: Option<Entity>,
  needed_percent_display: Option<Entity>,
  saved_percent_display: Option<Entity>,

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

    let frame_number = world.read_resource::<Time>().frame_number();
    if frame_number < UI_UPDATE_FRAMES {
      self.find_ui_components(world);
    } else if frame_number % UI_UPDATE_FRAMES == 0 {
      self.update_ui(world);
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
      needed_percent_display: None,
      saved_percent_display: None,
      name_display: None,
      description_display: None,
    }
  }

  fn update_name_and_description(&self, world: &mut World) {
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
  }

  fn update_fps(&self, world: &mut World) {
    if self.fps_display.is_some() {
      let mut ui_text = world.write_storage::<UiText>();
      if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
        let fps = world.read_resource::<FPSCounter>().sampled_fps();
        fps_display.text = format!("FPS: {:.*}", 1, fps);
      }
    }
  }

  fn update_spawn_stats(&mut self, world: &mut World) {
    if let (Some(spawned_display), Some(needed_percent_display), Some(saved_percent_display)) =
      (self.spawned_display, self.needed_percent_display, self.saved_percent_display)
    {
      let mut ui_text = world.write_storage::<UiText>();
      let spawn_stats = world.read_resource::<SpawnStats>();

      if let Some(spawned_display) = ui_text.get_mut(spawned_display) {
        spawned_display.text = format!("SPAWNED: {}/{}", spawn_stats.spawned, spawn_stats.total);
      }

      if let Some(needed_percent_display) = ui_text.get_mut(needed_percent_display) {
        needed_percent_display.text = format!("% NEEDED: {:.*}", 0, spawn_stats.win_ratio * 100.0);
      }

      if let Some(saved_percent_display) = ui_text.get_mut(saved_percent_display) {
        saved_percent_display.text = format!("% SAVED: {:.*}", 0, spawn_stats.saved_ratio() * 100.0);
      }
    }
  }

  fn update_ui(&mut self, world: &mut World) {
    self.update_name_and_description(world);
    self.update_fps(world);
    self.update_spawn_stats(world);
  }

  fn find_ui_components(&mut self, world: &mut World) {
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

    if self.needed_percent_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("needed_percent") {
          self.needed_percent_display = Some(entity);
        }
      });
    }

    if self.saved_percent_display.is_none() {
      world.exec(|finder: UiFinder| {
        if let Some(entity) = finder.find("saved_percent") {
          self.saved_percent_display = Some(entity);
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