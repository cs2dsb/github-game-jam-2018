use amethyst::ecs::prelude::*;

use ::{
  components::{
    Walker,
    Collider,
    Exit as ExitComponent,
    Remove,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
};

#[derive(Default)]
pub struct Exit;

impl<'s> System<'s> for Exit {
  type SystemData = (
    ReadStorage<'s, Walker>,
    ReadStorage<'s, ExitComponent>,
    ReadStorage<'s, Collider>,
    Read<'s, PhysicsWorld>,
    Write<'s, SpawnStats>,
    WriteStorage<'s, Remove>,
  );

  fn run(&mut self, (walkers, exit_components, colliders, physics_world, mut spawn_stats, mut remove): Self::SystemData) {
    //Go through fetching all sensors and checking if walkers are in proximity
    for (_ec, sensor) in (&exit_components, &colliders).join() {
      //Go through all other colliders in it's proximity
      if let Some(proxs) = physics_world.get_proximity(&sensor.collider_handle) {
        for prox in proxs {
          if let Some(entity) = physics_world.get_entity_for_collider(prox) {
            if let Some(_) = walkers.get(entity) {
              spawn_stats.saved += 1;
              remove
                .insert(entity, Remove)
                .expect("Failed to insert Remove component");
            }
          }
        }
      }
    }
  }
}