use amethyst::{
  core::{
    transform::components::Transform,
    cgmath::Vector2,
    timing::Time,
  },
  ecs::prelude::*,
  controls::FlyControlTag,
};

use ::{
  config::{
    SpawnerConfig,
    LevelsConfig,
    LevelConfig,
    CameraOverrides,
    CameraConfig,
    load_game_config,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
    Command,
    CommandChannel,
    Level as LevelResource,
    LoadState,
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
  RigidBodyCollider,
  Sensor,
}

///Manages the level. Reloads the config file containing levels, creates and destroys entities when the level starts and ends.
#[derive(Default)]
pub struct Level {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for Level {
  type SystemData = (
    Read<'s, CommandChannel>,
    Read<'s, Time>,
    Read<'s, LazyUpdate>,
    Write<'s, LevelResource>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (commands, time, updater, mut level_resource): Self::SystemData) {
    level_resource.runtime += time.delta_seconds();

    let mut pending_action = true;
    match level_resource.load_state {
      LoadState::NeedConfig => updater.exec_mut(move |world| load_config(world)),
      LoadState::NeedLoad => updater.exec_mut(move |world| load_level(world)),
      //The purpose of this state is purely to delay this system by 1 frame before load
      //so that the physics system has a chance to delete the colliders.
      //Without this, newly created colliders can collide with deleted ones for a
      //single frame and cause the simulation to try and separate them.
      LoadState::PhysicsCleanup => level_resource.load_state = LoadState::NeedLoad,
      LoadState::Loaded => pending_action = false,
    }

    if !pending_action {
      let mut reload = false;
      let mut next = false;
      let mut prev = false;
      let mut restart = false;
      for command in commands.read(self.command_reader.as_mut().unwrap()) {
        match command {
          Command::ReloadLevels => reload = true,
          Command::RestartLevel => restart = true,
          Command::NextLevel => next = true,
          Command::PreviousLevel => prev = true,
          _ => {},
        }
      }

      if reload {
        updater.exec_mut(move |world| reload_config(world));
      } else if next {
        updater.exec_mut(move |world| next_level(world));
      } else if prev {
        updater.exec_mut(move |world| prev_level(world));
      } else if restart {
        updater.exec_mut(move |world| restart_level(world));
      }
    }
  }
}

//Gets LevelsConfig resource and populates LevelResource with it
fn load_config(world: &mut World) {
  if LoadState::NeedConfig != world.read_resource::<LevelResource>().load_state {
    panic!("load_config called but load_state wasn't NeedConfig");
  }

  info!("Loading levels config");
  let level_config = world.read_resource::<LevelsConfig>();

  //Do some basic checks on the config
  if level_config.levels.len() == 0 {
    panic!("No levels defined in levels config");
  }
  let start_level = level_config.start_level.unwrap_or(0);
  if level_config.levels.len() <= start_level {
    panic!("start_level > max level");
  }

  //Update the level resource
  let mut level_resource = world.write_resource::<LevelResource>();
  level_resource.levels = level_config.levels.clone();
  //Make sure current level isn't higher than max level
  level_resource.current_level = level_resource.current_level.min(level_resource.levels.len() - 1);

  //This will trigger load on the next frame
  level_resource.load_state = LoadState::NeedLoad;
}

//Creates the entities associated with LevelResource.levels[LevelResource.current_level]
fn load_level(world: &mut World) {
  if LoadState::NeedLoad != world.read_resource::<LevelResource>().load_state {
    panic!("load_level called but load_state wasn't NeedLoad");
  }

  info!("Loading level");

  let prev_cam = {
    //TODO: must be a better way than the clone
    //Clone because level_resource is borrowed from world and we also need to mutate world
    let level = {
      let level_resource = world.read_resource::<LevelResource>();
      level_resource.levels[level_resource.current_level].clone()
    };
    //Create the level contents
    create_level_objects(world, &level);

    //Update the camera if there are overrides
    if let Some(camera_overrides) = &level.camera_overrides {
      Some(update_camera(world, camera_overrides))
    } else {
      None
    }
  };

  let mut level_resource = world.write_resource::<LevelResource>();
  level_resource.prev_camera_settings = prev_cam;
  level_resource.load_state = LoadState::Loaded;
  level_resource.runtime = 0.0;
}

//Cleans up anything we've created
fn unload_level(world: &mut World) {
  if LoadState::Loaded != world.read_resource::<LevelResource>().load_state {
    panic!("unload_level called but load_state wasn't Loaded");
  }

  info!("Unloading level");

  let prev_cam = {
    let mut level_resource = world.write_resource::<LevelResource>();
    level_resource.prev_camera_settings.take()
  };
  //Restore the prev camera settings if there were any
  if let Some(prev_cam) = prev_cam {
    update_camera(world, &prev_cam);
  }

  //Delete any entities we created
  {
    let entities = world.entities();

    //Delete all colliders (this also covers lifts, deadly areas and change direction)
    let components = world.read_storage::<Collider>();
    for (e, _) in (&entities, &components).join() {
      entities
        .delete(e)
        .expect("Failed to delete entitiy");
    }

    //Reset spawn stats
    let mut stats = world.write_resource::<SpawnStats>();
    *stats = SpawnStats::default();
  }

  let mut level_resource = world.write_resource::<LevelResource>();
  level_resource.load_state = LoadState::PhysicsCleanup;
}

fn restart_level(world: &mut World) {
  unload_level(world);
}

fn next_level(world: &mut World) {
  if {
    let mut level_resource = world.write_resource::<LevelResource>();
    if level_resource.current_level < level_resource.levels.len() - 1 {
      level_resource.current_level += 1;
      true
    } else {
      false
    }
  } {
    unload_level(world);
  }
}

fn prev_level(world: &mut World) {
  if {
    let mut level_resource = world.write_resource::<LevelResource>();
    if level_resource.current_level > 0 {
      level_resource.current_level -= 1;
      true
    } else {
      false
    }
  } {
    unload_level(world);
  }
}

//Attempts to load LevelsConfig from the filesystem and push it into the LevelsConfig resource is load succeeds
fn reload_config(world: &mut World) {
  if LoadState::Loaded != world.read_resource::<LevelResource>().load_state {
    panic!("reload_config called but load_state wasn't Loaded");
  }

  info!("Reloading level config");
  match load_game_config() {
    Ok(new_config) => {
      {
        let mut config = world.write_resource::<LevelsConfig>();
        *config = new_config.levels;
      }

      //Clear up the current level
      unload_level(world);

      //Trigger a config load on next frame
      let mut level_resource = world.write_resource::<LevelResource>();
      level_resource.load_state = LoadState::NeedConfig;
    },
    Err(e) => error!("Error loading GameConfig: {}", e),
  }
}

fn create_object(world: &mut World, width: f32, height: f32, x: f32, y: f32, otype: ObjectType, color: Option<Color>, rotation: Option<f32>, add_extras: Option<&Fn(EntityBuilder) -> EntityBuilder>) {
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
      ObjectType::RigidBodyCollider =>
        physics_world.create_rigid_body_with_box_collider(
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

fn create_wall(world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
  create_object(
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

fn create_hazard(world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
  create_object(
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

fn create_exit(world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
  create_object(
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

fn create_block(world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>) {
  create_object(
    world,
    width,
    height,
    x,
    y,
    ObjectType::RigidBodyCollider,
    color,
    rotation,
    None,
  );
}

fn create_spawner(world: &mut World, width: f32, height: f32, x: f32, y: f32, color: Option<Color>, rotation: Option<f32>, freq: f32, max: u32) {
  world
    .write_resource::<SpawnStats>()
    .total += max;

  create_object(world,
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

fn create_level_objects(world: &mut World, level: &LevelConfig) {
  if let Some(ref set) = level.walls {
    for o in &set.list {
      create_wall(
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
      create_hazard(
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
      create_exit(
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
    let (freq, max) = {
      if let Some(ref overrides) = level.spawn_overrides {
        (overrides.freq, overrides.max)
      } else {
        let config = world.read_resource::<SpawnerConfig>();
        (config.frequency_default, config.max_default)
      }
    };

    for o in &set.list {
      create_spawner(
        world,
        o.size.x,
        o.size.y,
        o.position.x,
        o.position.y,
        o.color.or(set.color),
        o.rotation,
        freq,
        max,
      );
    }
  }

  if let Some(ref set) = level.blocks {
    for o in &set.list {
      create_block(
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

fn update_camera(world: &mut World, overrides: &CameraOverrides) -> CameraOverrides {
  let mut camera_config = world.write_resource::<CameraConfig>();
  let prev = CameraOverrides {
    convergence_speed: Some(camera_config.convergence_speed),
    offset: Some(camera_config.offset),
    //We don't want to restore the position
    position: None,
  };

  if let Some(convergence_speed) = &overrides.convergence_speed {
    camera_config.convergence_speed = *convergence_speed;
  }

  if let Some(offset) = &overrides.offset {
    camera_config.offset = *offset;
  }

  //TODO: This doesn't work on the first level because the FlyControlTag hasn't been initialized
  //due to async prefab loading in RunningState
  //It's not really an issue since the prefab sets the pos for the first level anyway.
  if let Some(position) = &overrides.position {
    for (t, _) in (
      &mut world.write_storage::<Transform>(),
      &world.read_storage::<FlyControlTag>()).join() {

      t.translation = *position;
    }
  }

  prev
}