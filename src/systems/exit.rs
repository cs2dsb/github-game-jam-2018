use amethyst::{
  ecs::prelude::*,
  assets::AssetStorage,
  audio::{
    Source,
    output::Output,
  },
};

use ::{
  components::{
    Walker,
    Collider,
    Exit as ExitComponent,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
    Sounds,
  },
};

#[derive(Default)]
pub struct Exit;

impl<'s> System<'s> for Exit {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Walker>,
    ReadStorage<'s, ExitComponent>,
    ReadStorage<'s, Collider>,
    Read<'s, PhysicsWorld>,
    Write<'s, SpawnStats>,
    ReadExpect<'s, Sounds>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
  );

  fn run(&mut self, (entities, walkers, exit_components, colliders, physics_world, mut spawn_stats, sounds, source_storage, output): Self::SystemData) {
    //Go through fetching all sensors and checking if walkers are in proximity
    for (_ec, sensor) in (&exit_components, &colliders).join() {
      //Go through all other colliders in it's proximity
      if let Some(proxs) = physics_world.get_proximity(&sensor.collider_handle) {
        for prox in proxs {
          if let Some(entity) = physics_world.get_entity_for_collider(prox) {
            if let Some(_) = walkers.get(entity) {
              spawn_stats.saved += 1;

              entities
                .delete(entity)
                .expect("Failed to delete entity");

              if let Some(output) = &output {
                sounds.play_exit(&source_storage, output);
              }
            }
          }
        }
      }
    }
  }
}