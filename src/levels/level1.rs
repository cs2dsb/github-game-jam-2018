use amethyst::{
  core::{
    cgmath::Vector2,
    transform::Transform,
  },
  ecs::prelude::*,
};

use ::{
  config::SpawnerConfig,
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
  components::{
    Color,
    Spawner,
    SpawnerParams,
  },
};

use super::Level;

pub struct Level1 {
  wall_color: Color,
}

impl Default for Level1 {
  fn default() -> Self {
    Self {
      wall_color: Color::new(0.4, 0.4, 0.4, 1.0),
    }
  }
}

impl Level for Level1 {
  fn create_entities(&self, world: &mut World) {
    self.create_spawner(world);
    self.create_walls(world);
    self.create_platforms(world);
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

  fn create_walls(&self, world: &mut World) {
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

    world
      .create_entity()
      .with(c0)
      .with(self.wall_color)
      .build();

    world
      .create_entity()
      .with(c1)
      .with(self.wall_color)
      .build();

    world
      .create_entity()
      .with(c2)
      .with(self.wall_color)
      .build();

    world
      .create_entity()
      .with(c3)
      .with(self.wall_color)
      .build();

  }

  fn create_platforms(&self, world: &mut World) {
    let c0 = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();

      let len = 200.0;
      let thickness = 20.0;

      //Bottom
      let c0 = physics_world.create_ground_box_collider(
        &Vector2::new(len/2.0, 100.0), //Pos
        &Vector2::new(len, thickness), //Size
        0.0);

      c0
    };

    world
      .create_entity()
      .with(c0)
      .with(self.wall_color)
      .build();
  }
}