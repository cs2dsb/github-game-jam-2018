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

//TODO: the lerping appears to do what you'd expect (if you set the timestep to a large number like 10 seconds
// you can see the objects move along the path between steps). HOWEVER, physics objects still jerk around
// sometimes. It's very visible if you turn logging up to debug; writing to stdout appears to be causing hitches.
// I thought fixed timesteps + lerping would smooth this out but apparently not.
// I haven't spent too long on it as there is work ongoing to integrate nphysics and specs properly and when that
// lands much/all of this will be obsolete.

///Updates the transforms of entities with colliders. Peforms lerping between n-1 and n frame.
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