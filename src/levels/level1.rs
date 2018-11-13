use amethyst::{
  core::{
    cgmath::Vector2,
    transform::Transform,
  },
  ecs::prelude::*,
  renderer::Shape,
};

use ::{
  config::{
    SpawnerConfig,
    PhysicsConfig,
    LevelsConfig,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
  components::{
    Color,
    Spawner,
    SpawnerParams,
    Shape as ShapeComponent,
    Exit,
    DeadlyArea,
  },
};

use super::Level;

pub struct Level1 {
  wall_color: Color,
}

impl Default for Level1 {
  fn default() -> Self {
    Self {
      wall_color: Color::new(0.6, 0.6, 0.6, 1.0),
    }
  }
}

impl Level for Level1 {
  fn create_entities(&self, world: &mut World) {
    self.create_spawner(world);
    self.create_walls(world);
    self.create_exits(world);
    self.create_hazards(world);
  }
}

impl Level1 {
  fn create_spawner(&self, world: &mut World) {
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

  fn create_exits(&self, world: &mut World) {
    let (width, height) = {
      let physics_config = world.read_resource::<PhysicsConfig>();
      (physics_config.exit_width, physics_config.exit_height)
    };

    let shape = ShapeComponent {
      shape: Shape::IcoSphere(None),
      scale: (width * 0.5, height * 0.5, 0.1),
    };

    let mut transform = Transform::default();
    transform.translation.x = 480.0;
    transform.translation.y = 130.0;

    let sensor = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();
      physics_world.create_ground_box_sensor(
        &Vector2::new(transform.translation.x, transform.translation.y), //Pos
        &Vector2::new(width, height), //Size
      0.0)
    };

    let color = Color::new(0.2, 0.8, 0.2, 1.0);

    world
      .create_entity()
      .with(Exit)
      .with(shape)
      .with(transform)
      .with(sensor)
      .with(color)
      .build();
  }



  fn create_wall(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32) {
    let collider = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();
      physics_world.create_ground_box_collider(
        &Vector2::new(x, y),
        &Vector2::new(width, height),
      0.0)
    };

    let color = self.wall_color.clone();

    world
      .create_entity()
      .with(collider)
      .with(color)
      .build();
  }

  fn create_hazard(&self, world: &mut World, width: f32, height: f32, x: f32, y: f32) {
    let shape = ShapeComponent {
      shape: Shape::Cube,
      scale: (width * 0.5, height * 0.5, 0.1),
    };

    let mut transform = Transform::default();
    transform.translation.x = x;
    transform.translation.y = y;

    let sensor = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();
      physics_world.create_ground_box_sensor(
        &Vector2::new(x, y),
        &Vector2::new(width, height),
      0.0)
    };

    let color = Color::new(0.8, 0.2, 0.2, 1.0);

    world
      .create_entity()
      .with(DeadlyArea)
      .with(shape)
      .with(transform)
      .with(sensor)
      .with(color)
      .build();
  }

  fn create_hazards(&self, world: &mut World) {
    let mut list = Vec::new();
    //This is to get around the world borrow //TODO: better way?
    {
      let levels_config = world.read_resource::<LevelsConfig>();
      if levels_config.levels.len() < 1 {
        panic!("LevelsConfig.levels.len < 1");
      }

      if let Some(ref hazards) = levels_config.levels[0].deadly_areas {
        for h in &hazards.vec {
          list.push(h.clone());
        }
      }
    }

    for h in list {
      self.create_hazard(world, h.size.x, h.size.y, h.position.x, h.position.y);
    }
  }

  fn create_walls(&self, world: &mut World) {
    let mut list = Vec::new();
    //This is to get around the world borrow //TODO: better way?
    {
      let levels_config = world.read_resource::<LevelsConfig>();
      if levels_config.levels.len() < 1 {
        panic!("LevelsConfig.levels.len < 1");
      }

      if let Some(ref walls) = levels_config.levels[0].walls {
        for h in &walls.vec {
          list.push(h.clone());
        }
      }
    }

    for h in list {
      self.create_wall(world, h.size.x, h.size.y, h.position.x, h.position.y);
    }
  }
}