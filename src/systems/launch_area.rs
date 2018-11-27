use std::cmp::PartialEq;

use amethyst::{
  ecs::prelude::*,
  assets::AssetStorage,
  audio::{
    Source,
    output::Output,
  },
};

use nphysics2d::{
  object::BodyHandle,
  math::{
    Velocity,
  },
};

use nalgebra::{
  Vector2 as naVector2,
};

use ::{
  config::PhysicsConfig,
  components::{
    Walker,
    Collider,
    LaunchArea as LaunchAreaComponent,
    Direction,
  },
  resources::{
    PhysicsWorld,
    Sounds,
  },
};

///Exists to provide PartialEq so the comparrison only looks at the handle
#[derive(Debug)]
struct ToLaunch {
  direction: Direction,
  handle: BodyHandle,
}

impl PartialEq for ToLaunch {
  fn eq(&self, rhs: &Self) -> bool {
    self.handle.eq(&rhs.handle)
  }
}

///Checks proximity events between launch areas and walkers and applies a velocity to the walkers that overlap.
#[derive(Default)]
pub struct LaunchArea;

impl<'s> System<'s> for LaunchArea {
  type SystemData = (
    ReadStorage<'s, Walker>,
    ReadStorage<'s, LaunchAreaComponent>,
    ReadStorage<'s, Collider>,
    Write<'s, PhysicsWorld>,
    Read<'s, PhysicsConfig>,
    ReadExpect<'s, Sounds>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
  );

  fn run(&mut self, (walkers, launch_area_components, colliders, mut physics_world, physics_config, sounds, source_storage, output): Self::SystemData) {
    let mut to_launch = Vec::new();

    //Go through fetching all sensors and checking if walkers are in proximity
    for (launch_area, sensor) in (&launch_area_components, &colliders).join() {
      //Go through all other colliders in it's proximity
      if let Some(proxs) = physics_world.get_proximity(&sensor.collider_handle) {
        for prox in proxs {
          if let Some(entity) = physics_world.get_entity_for_collider(prox) {
            //Only works on walkers
            if let Some(_) = walkers.get(entity) {
              //We want to change the velocity of these but physics_world is already borrowed
              //not sure if there's a better way to do this...
              if let Some(body_handle) = physics_world.get_body_for_collider(prox) {
                let tl = ToLaunch {
                  direction: launch_area.direction,
                  handle: *body_handle,
                };
                if !to_launch.contains(&tl) {
                  to_launch.push(tl);
                }
              }
            }
          }
        }
      }
    }

    //No point playing the same sound multiple times in the same frame
    if to_launch.len() > 0 {
      if let Some(output) = &output {
        //TODO: current this sound gets played a bunch of times because the overlap happens for more than one frame.
        sounds.play_lift(&source_storage, output);
      }

      let velocity_left = Velocity::new(
        naVector2::new(
          physics_config.lift_velocity.x * -1.0,
          physics_config.lift_velocity.y,
        ),
        physics_config.lift_velocity_rotation * -1.0,
      );

      let velocity_right = Velocity::new(
        naVector2::new(
          physics_config.lift_velocity.x,
          physics_config.lift_velocity.y,
        ),
        physics_config.lift_velocity_rotation,
      );

      //TODO: Would be interesting to benchmark while(pop) vs ref iter + derefing the handle
      while let Some(ToLaunch {direction, handle}) = to_launch.pop() {
        if let Some(body) = physics_world.world.rigid_body_mut(handle) {
          match direction {
            Direction::Left => body.set_velocity(velocity_left),
            Direction::Right => body.set_velocity(velocity_right),
          }
        }
      }
    }
  }
}