use amethyst::ecs::prelude::*;

use ::{
  components::{
    Walker,
    Collider,
    DeadlyArea as DeadlyAreaComponent,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
};

#[derive(Default)]
pub struct DeadlyArea;

impl<'s> System<'s> for DeadlyArea {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Walker>,
    ReadStorage<'s, DeadlyAreaComponent>,
    ReadStorage<'s, Collider>,
    Read<'s, PhysicsWorld>,
    Write<'s, SpawnStats>,
  );

  fn run(&mut self, (entities, walkers, deadly_area_components, colliders, physics_world, mut spawn_stats): Self::SystemData) {
    //Go through fetching all sensors and checking if walkers are in proximity
    for (_ec, sensor) in (&deadly_area_components, &colliders).join() {
      //Go through all other colliders in it's proximity
      if let Some(proxs) = physics_world.get_proximity(&sensor.collider_handle) {
        for prox in proxs {
          if let Some(entity) = physics_world.get_entity_for_collider(prox) {
            if let Some(_) = walkers.get(entity) {
              spawn_stats.killed += 1;
              entities
                .delete(entity)
                .expect("Failed to delete entity");
            }
          }
        }
      }
    }
  }
}