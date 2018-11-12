use std::f32::consts::PI;
use rand::random;

 use amethyst::{
  assets::{
    Prefab,
    Handle,
  },
  core::{
    Time,
    cgmath::{
      Vector2,
    },
    transform::Transform,
  },
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
  config::SpawnerConfig,
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
  components::{
    Spawner,
    SpawnerParams,
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
}

impl<'a, 'b> SimpleState<'a, 'b> for RunningState {
  fn on_start(&mut self, data: StateData<GameData>) {
    info!("RunningState.on_start");
    let world = data.world;

    self.initialise_prefab(world);
    self.initialise_ui(world);

    self.test_physics(world);
    self.test_spawner(world);
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

  fn test_spawner(&self, world: &mut World) {
    let (freq, max) = {
      let config = world.read_resource::<SpawnerConfig>();
      (config.frequency_default, config.max_default)
    };

    let spawner = Spawner::new(SpawnerParams {
      spawn_size: Vector2::new(10.0, 10.0),
      spawn_max: max,
      frequency: freq,
    });
    let mut spawner_transform = Transform::default();
    spawner_transform.translation.x = 30.0;
    spawner_transform.translation.y = 30.0;

    world
      .write_resource::<SpawnStats>()
      .total += spawner.spawn_max;

    world
      .create_entity()
      .with(spawner)
      .with(spawner_transform)
      .build();
  }

  fn test_physics(&self, world: &mut World) {
    let (c0, c1, c2, c3) = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();

      let len = 1000.0;
      let thickness = 10.0;


      //Bottom
      let c0 = physics_world.create_ground_box_collider(
        &Vector2::new(len/2.0, thickness/2.0), //Pos
        &Vector2::new(len, thickness), //Size
        0.0);

      //Top
      let c1 = physics_world.create_ground_box_collider(
        &Vector2::new(len/2.0, len-thickness/2.0), //Pos
        &Vector2::new(len, thickness), //Size
        0.0);

      //Left
      let c2 = physics_world.create_ground_box_collider(
        &Vector2::new(thickness/2.0, len/2.0), //Pos
        &Vector2::new(thickness, len), //Size
        0.0);

      //Right
      let c3 = physics_world.create_ground_box_collider(
        &Vector2::new(len-thickness/2.0, len/2.0), //Pos
        &Vector2::new(thickness, len), //Size
        0.0);

      (c0, c1, c2, c3)
    };

    world.create_entity().with(c0).build();
    world.create_entity().with(c1).build();
    world.create_entity().with(c2).build();
    world.create_entity().with(c3).build();

    for i in 1..10 {
      //Collider attached to dynamic body
      let collider = {
        let mut physics_world = world.write_resource::<PhysicsWorld>();
        physics_world.create_rigid_body_with_box_collider(
          &Vector2::new(64.0, 1000.0 + 200.0 * (i as f32)),
          &Vector2::new(64.0, 64.0),
          random::<f32>() * 360.0 * PI / 180.0)
      };

      world
        .create_entity()
        .with(collider)
        .build();
    }
  }
}