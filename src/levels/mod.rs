use amethyst::{
  core::{
    cgmath::Vector2,
  },
  ecs::prelude::*,
};

use ::{
  config::{
    SpawnerConfig,
    LevelsConfig,
    LevelConfig,
    CameraOverrides,
    CameraConfig,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
  components::{
    Color,
    Spawner,
    SpawnerParams,
    Exit,
    DeadlyArea,
    Collider,
  },
};

enum ObjectType {
  GroundCollider,
  Sensor,
}

#[derive(Default)]
pub struct Level {
  current_level: usize,
  levels: Vec<LevelConfig>,
  loaded: bool,
  prev_camera_settings: Option<CameraOverrides>,
}

impl Level {
  pub fn new(level_config: &LevelsConfig) -> Self {
    if level_config.levels.len() == 0 {
      panic!("No levels defined in levels config");
    }
    let start_level = level_config.start_level.unwrap_or(0);
    if level_config.levels.len() <= start_level {
      panic!("start_level > max level");
    }
    Self {
      current_level: start_level,
      loaded: false,
      levels: level_config.levels.clone(),
      prev_camera_settings: None,
    }
  }

  pub fn load(&mut self, world: &mut World) {
    if self.loaded {
      self.unload(world);
    }
    let level = &self.levels[self.current_level];
    if let Some(camera_overrides) = &level.camera_overrides {
      self.prev_camera_settings = Some(self.update_camera(world, camera_overrides));
    }
    self.create_level_objects(world, level);
    self.loaded = true;
  }

  pub fn next(&mut self) {
    if self.is_more_levels() {
      self.current_level += 1;
    }
  }

  pub fn name(&self) -> String {
    if let Some(ref name) = self.levels[self.current_level].name {
      name.clone()
    } else {
      "".to_string()
    }
  }

  pub fn description(&self) -> String {
    if let Some(ref description) = self.levels[self.current_level].description {
      description.clone()
    } else {
      "".to_string()
    }
  }

  pub fn is_more_levels(&self) -> bool {
    self.current_level < (self.levels.len() - 1)
  }

  fn update_camera(&self, world: &mut World, overrides: &CameraOverrides) -> CameraOverrides {
    let mut camera_config = world.write_resource::<CameraConfig>();
    let prev = CameraOverrides {
      convergence_speed: Some(camera_config.convergence_speed),
      offset: Some(camera_config.offset),
    };

    if let Some(convergence_speed) = &overrides.convergence_speed {
      camera_config.convergence_speed = *convergence_speed;
    }
    if let Some(offset) = &overrides.offset {
      camera_config.offset = *offset;
    }

    prev
  }

  pub fn unload(&mut self, world: &mut World) {
    if self.loaded {
      self.loaded = false;

      //Restore the previous camera settings if there are any
      if let Some(prev_camera_settings) = self.prev_camera_settings.take() {
        self.update_camera(world, &prev_camera_settings);
      }

      {
        let entities = world.entities();

        //This is to make sure we don't double delete entities
        let mut to_delete = Vec::new();

        //Delete all existing colliders (this also covers lifts, deadly areas and change direction)
        let components = world.read_storage::<Collider>();
        for (e, _) in (&entities, &components).join() {
          if !to_delete.contains(&e) {
            to_delete.push(e);
          }
        }

        for e in to_delete {
          entities
            .delete(e)
            .expect("Failed to delete entitiy");
        }

        //Reset spawn stats
        let mut stats = world.write_resource::<SpawnStats>();
        *stats = SpawnStats::default();
      }
      //To make sure entities are actually deleted before we proceed
      world.maintain();
    }
  }

  fn create_object(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, otype: ObjectType, color: Option<Color>, rotation: Option<f32>, add_extras: Option<&Fn(EntityBuilder) -> EntityBuilder>) {
    let object = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();
      match otype {
        ObjectType::GroundCollider =>
          physics_world.create_ground_box_collider(
            &Vector2::new(x, y),
            &Vector2::new(width, height),
            rotation.unwrap_or(0.0)),
        ObjectType::Sensor =>
          physics_world.create_ground_box_sensor(
            &Vector2::new(x, y),
            &Vector2::new(width, height),
            rotation.unwrap_or(0.0)),
      }
    };

    let mut builder = world.create_entity();
    builder = builder.with(object);

    if let Some(color) = color {
      builder = builder.with(color);
    }

    if let Some(add_extras) = add_extras {
      builder = add_extras(builder);
    }

    builder.build();
  }


  fn create_wall(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
    self.create_object(
      world,
      width,
      height,
      x,
      y,
      ObjectType::GroundCollider,
      color,
      rotation,
      None
    );
  }

  fn create_hazard(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
    self.create_object(
      world,
      width,
      height,
      x,
      y,
      ObjectType::Sensor,
      color,
      rotation,
      Some(&|builder| builder.with(DeadlyArea)),
    );
  }

  fn create_exit(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
    self.create_object(
      world,
      width,
      height,
      x,
      y,
      ObjectType::Sensor,
      color,
      rotation,
      Some(&|builder| builder.with(Exit)),
    );
  }

  fn create_spawner(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
    let (freq, max) = {
      let config = world.read_resource::<SpawnerConfig>();
      (config.frequency_default, config.max_default)
    };

    world
      .write_resource::<SpawnStats>()
      .total += max;

    self.create_object(world,
      width,
      height,
      x,
      y,
      ObjectType::Sensor,
      color,
      rotation,
      Some(&|builder| {
        let spawner = Spawner::new(SpawnerParams {
          spawn_size: Vector2::new(10.0, 10.0),
          spawn_max: max,
          frequency: freq,
        });
        builder.with(spawner)
      }),
    );
  }

  fn create_level_objects(&self, world: &mut World, level: &LevelConfig) {
    if let Some(ref set) = level.walls {
      for o in &set.list {
        self.create_wall(
          world,
          o.size.x,
          o.size.y,
          o.position.x,
          o.position.y,
          o.color.or(set.color),
          o.rotation,
        );
      }
    }

    if let Some(ref set) = level.deadly_areas {
      for o in &set.list {
        self.create_hazard(
          world,
          o.size.x,
          o.size.y,
          o.position.x,
          o.position.y,
          o.color.or(set.color),
          o.rotation,
        );
      }
    }

    if let Some(ref set) = level.exits {
      for o in &set.list {
        self.create_exit(
          world,
          o.size.x,
          o.size.y,
          o.position.x,
          o.position.y,
          o.color.or(set.color),
          o.rotation,
        );
      }
    }

    if let Some(ref set) = level.spawners {
      for o in &set.list {
        self.create_spawner(
          world,
          o.size.x,
          o.size.y,
          o.position.x,
          o.position.y,
          o.color.or(set.color),
          o.rotation,
        );
      }
    }
  }
}