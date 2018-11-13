use amethyst::ecs::prelude::*;

use ::{
  components::{
    Family as FamilyComponent,
    Matriarch,
    Remove as RemoveComponent,
  },
};

#[derive(Default)]
pub struct Remove;

impl<'s> System<'s> for Remove {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, FamilyComponent>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, RemoveComponent>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (entities, family_components, matriarchs, remove, updater): Self::SystemData) {
    for (e, _) in (&entities, &remove).join() {
      //Nothing to do if it's already dead
      if !entities.is_alive(e) {
        continue;
      }

      //Extra logic if it's a matriarch
      if let (Some(_), Some(fam)) = (matriarchs.get(e), family_components.get(e)) {
        if let Some(next) = fam.next {
          info!("Promoted {:?} to Matriarch", next);
          updater.insert(next, Matriarch);
        }
      }

      entities
        .delete(e)
        .expect("Failed to delete entity");
    }
  }
}