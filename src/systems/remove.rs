use amethyst::ecs::prelude::*;

use ::{
  components::{
    Remove as RemoveComponent,
  },
};

#[derive(Default)]
pub struct Remove;

impl<'s> System<'s> for Remove {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, RemoveComponent>,
  );

  fn run(&mut self, (entities, remove): Self::SystemData) {
    for (e, _) in (&entities, &remove).join() {
      //Nothing to do if it's already dead
      if !entities.is_alive(e) {
        continue;
      }

      entities
        .delete(e)
        .expect("Failed to delete entity");
    }
  }
}