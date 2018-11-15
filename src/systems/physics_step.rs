use std::collections::HashMap;

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
  collider_cache: HashMap<usize, Collider>,
}

impl<'s> System<'s> for PhysicsStep {
  type SystemData = (
    Entities<'s>,
    Read<'s, Time>,
    Write<'s, PhysicsWorld>,
    WriteStorage<'s, Collider>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    let mut storage: WriteStorage<Collider> = SystemData::fetch(&res);
    self.inserted_reader_id = Some(storage.track_inserted());
    self.removed_reader_id = Some(storage.track_removed());
  }

  fn run(&mut self, (entities, time, mut physics_world, mut colliders): Self::SystemData) {
    let delta = time.delta_seconds();
    physics_world.add_time(delta);

    while physics_world.step() {
      //This is done here rather than the transform update system because c.update_transform must be called per physics step
      for c in (&mut colliders).join() {
        let collider = physics_world
          .world
          .collider(c.collider_handle)
          .expect("Failed to resolve collider handle to collider");

        c.update_transform(&collider.position());
      }
    }

    //Do removed first to stop deleting then creating an entity in the same frame from triggering the
    //panic below (when index is already used)
    self.dirty.clear();
    colliders.populate_removed(&mut self.removed_reader_id.as_mut().unwrap(), &mut self.dirty);

    for (index) in (&self.dirty).join() {
      let index = index as usize;
      if let Some(collider) = self.collider_cache.remove(&index) {
        physics_world.destroy_collider(collider);
        debug!("Collider was deleted: {:?}", index);
      } else {
        //Changed this from panic to warning as it was happening around level reload/next
        //Either think more carefully about how to keep these in sync reliably or wait for
        //specs issue 361 to be resolved.
        warn!("Collider index {:?} missing from collider_cache", index);
      }
    }

    self.dirty.clear();
    colliders.populate_inserted(&mut self.inserted_reader_id.as_mut().unwrap(), &mut self.dirty);

    for (e, c, index) in (&entities, &colliders, &self.dirty).join() {
      //These checks are recommended in the specs book on flagged storage
      if !entities.is_alive(e) || !colliders.contains(e) {
        continue;
      }
      let index = index as usize;
      if self.collider_cache.contains_key(&index) {
        //TODO:
        // https://github.com/slide-rs/specs/issues/361 talks about the issue of not being able
        // to get the entity/component with/after the remove event. Whatever they decide will
        // likely replace what I've cobbled together here
        panic!("Collider created with the same index as an existing collider");
      }
      self.collider_cache.insert(index, c.clone());
      physics_world.register_entity(e, c.collider_handle);
    }

  }
}