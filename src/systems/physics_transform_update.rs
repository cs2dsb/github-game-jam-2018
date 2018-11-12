///Finds colliders without meshes and creates meshes for them

use amethyst::{
  core::transform::components::Transform,
  ecs::prelude::*,
};

use ::{
  components::{
    Collider,
  },
  resources::{
    PhysicsWorld,
  },
};

#[derive(Default)]
pub struct PhysicsTransformUpdate;

impl<'s> System<'s> for PhysicsTransformUpdate {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, Collider>,
    WriteStorage<'s, Transform>,
    Write<'s, PhysicsWorld>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (entities, mut colliders, mut transforms, physics_world, updater): Self::SystemData) {
    //Create transforms for colliders that don't have them
    for (entity, c, _) in (&entities, &mut colliders, !&transforms).join() {
      let collider = physics_world
        .world
        .collider(c.collider_handle)
        .expect("Failed to resolve collider handle to collider");

      //Update the colliders transforms
      c.update_transform(&collider.position());
      //Insert a transform for it
      updater.insert(entity, c.transform_next.clone());
    }

    //This doesn't include the difference between physics_step and now but I benchmarked it and
    //that turned out to be a handful of nanoseconds so not worth worrying about.
    let alpha = physics_world.get_alpha();
    //Update the transform based off the colliders position
    for (c, transform) in (&colliders, &mut transforms).join() {
      c.lerp_transform(transform, alpha);
    }
  }
}