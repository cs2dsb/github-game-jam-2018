use std::collections::BTreeMap;

use amethyst::{
  core::{
    //transform::components::Transform,
    timing::Time,
  },
  ecs::prelude::*,
};

use ::{
  components::Collider,
  resources::PhysicsWorld,
};

///System steps the physics world. Also manages deleting colliders when their components get removed.
#[derive(Default)]
pub struct PhysicsStep {
  dirty: BitSet,
  inserted_reader_id: Option<ReaderId<InsertedFlag>>,
  removed_reader_id: Option<ReaderId<RemovedFlag>>,
  //Keeps a copy of all colliders so they can be deleted when a remove event is received
  collider_cache: BTreeMap<usize, Collider>,
}

impl<'s> System<'s> for PhysicsStep {
  type SystemData = (
    Read<'s, Time>,
    Write<'s, PhysicsWorld>,
    ReadStorage<'s, Collider>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    let mut storage: WriteStorage<Collider> = SystemData::fetch(&res);
    self.inserted_reader_id = Some(storage.track_inserted());
    self.removed_reader_id = Some(storage.track_removed());
  }

  fn run(&mut self, (time, mut physics_world, colliders): Self::SystemData) {
    self.dirty.clear();
    colliders.populate_inserted(&mut self.inserted_reader_id.as_mut().unwrap(), &mut self.dirty);

    for (c, index) in (&colliders, &self.dirty).join() {
      let index = index as usize;
      if self.collider_cache.contains_key(&index) {
        //TODO:
        //  This should be prevented by the generational index in specs but since the FlaggedStorage doesn't give us
        //  I'm not sure how to to do better. https://github.com/slide-rs/specs/issues/361 talks about the issue of not
        //  being able to get the entity/component with/after the remove event. Whatever they decide will likely replace
        //  what I've cobbled together here
        panic!("Collider created with the same index as an existing collider");
      }
      self.collider_cache.insert(index, c.clone());
      info!("Collider was inserted: {:?}", index);
    }

    self.dirty.clear();
    colliders.populate_removed(&mut self.removed_reader_id.as_mut().unwrap(), &mut self.dirty);

    for (index) in (&self.dirty).join() {
      let index = index as usize;
      if let Some(collider) = self.collider_cache.remove(&index) {
        let mut found = false;
        for (_, c) in &self.collider_cache {
          if c.body_handle == collider.body_handle {
            found = true;
            break;
          }
        }
        physics_world.destroy_collider(collider, !found);
        info!("Collider was deleted: {:?}", index);
      } else {
        panic!("Collider index {:?} missing from collider_cache", index);
      }
    }


    let delta = time.delta_seconds();
    physics_world.step(delta);

    //for (transform, velocity) in (&mut transforms, &basic_velocities).join() {
    //  transform.translation += velocity.velocity * delta;
    //}
  }
}