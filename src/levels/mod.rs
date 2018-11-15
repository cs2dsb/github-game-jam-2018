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
}

impl Level {
  pub fn new(level_config: &LevelsConfig) -> Self {
    if level_config.levels.len() == 0 {
      panic!("No levels defined in levels config");
    }
    Self {
      current_level: 0,
      levels: level_config.levels.clone(),
    }
  }

  pub fn load(&self, world: &mut World) {
    if self.current_level > 0 {
      self.unload(world);
    }

    let level = &self.levels[self.current_level];
    self.create_level_objects(world, level);
  }

  pub fn next(&mut self) {
    self.current_level += 1;
  }

  pub fn is_more_levels(&self) -> bool {
    self.current_level < self.levels.len()
  }

  fn unload(&self, world: &mut World) {
    let entities = world.entities();

    //Delete all existing colliders
    let colliders = world.read_storage::<Collider>();
    for (e, _) in (&entities, &colliders).join() {
      entities
        .delete(e)
        .expect("Failed to delete entity");
    }

    //Reset spawn stats
    let mut stats = world.write_resource::<SpawnStats>();
    *stats = SpawnStats::default();
  }

  fn create_object(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, otype: ObjectType, color: Option<Color>, add_extras: Option<&Fn(EntityBuilder) -> EntityBuilder>) {
    let object = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();
      match otype {
        ObjectType::GroundCollider =>
          physics_world.create_ground_box_collider(
            &Vector2::new(x, y),
            &Vector2::new(width, height),
            0.0),
        ObjectType::Sensor =>
          physics_world.create_ground_box_sensor(
            &Vector2::new(x, y),
            &Vector2::new(width, height),
            0.0),
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


  fn create_wall(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>) {
    self.create_object(
      world,
      width,
      height,
      x,
      y,
      ObjectType::GroundCollider,
      color,
      None
    );
  }

  fn create_hazard(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>) {
    self.create_object(
      world,
      width,
      height,
      x,
      y,
      ObjectType::Sensor,
      color,
      Some(&|builder| builder.with(DeadlyArea)),
    );
  }

  fn create_exit(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>) {
    self.create_object(
      world,
      width,
      height,
      x,
      y,
      ObjectType::Sensor,
      color,
      Some(&|builder| builder.with(Exit)),
    );
  }

  fn create_spawner(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>) {
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
        );
      }
    }
  }
}