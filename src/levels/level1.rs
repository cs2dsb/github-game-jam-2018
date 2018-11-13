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
    self.create_platforms(world);
    self.create_exits(world);
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
    let mut platforms = Vec::new();
    {
      let mut physics_world = world.write_resource::<PhysicsWorld>();

      //World is 1000
      let len = 100.0;
      let thickness = 10.0;

      platforms.push(physics_world.create_ground_box_collider(
        &Vector2::new(len, len), //Pos
        &Vector2::new(len*2.0, thickness), //Size
        0.0));

      platforms.push(physics_world.create_ground_box_collider(
        &Vector2::new(len*4.0, len), //Pos
        &Vector2::new(len*2.0, thickness), //Size
        0.0));

      platforms.push(physics_world.create_ground_box_collider(
        &Vector2::new(len*4.0, len * 0.5), //Pos
        &Vector2::new(len, len), //Size
        0.0));

      platforms.push(physics_world.create_ground_box_collider(
        &Vector2::new(len*4.0, len * 2.0), //Pos
        &Vector2::new(len, len), //Size
        0.0));
    }

    while let Some(c) = platforms.pop() {
      world
        .create_entity()
        .with(c)
        .with(self.wall_color)
        .build();
    }
  }

  fn create_exits(&self, world: &mut World) {
    let (width, height) = {
      let physics_config = world.read_resource::<PhysicsConfig>();
      (physics_config.exit_width, physics_config.exit_height)
    };

    let shape = ShapeComponent {
      shape: Shape::IcoSphere(None),
      scale: (width * 0.5, height * 0.5, width * 0.5),
    };

    let mut transform = Transform::default();
    transform.translation.x = 500.0;
    transform.translation.y = 200.0;

    let sensor = {
      let mut physics_world = world.write_resource::<PhysicsWorld>();
      physics_world.create_ground_box_sensor(
        &Vector2::new(transform.translation.x, transform.translation.y), //Pos
        &Vector2::new(width * 0.5, height * 0.5), //Size
      0.0)
    };

    world
      .create_entity()
      .with(Exit)
      .with(shape)
      .with(transform)
      .with(sensor)
      .build();
  }
}