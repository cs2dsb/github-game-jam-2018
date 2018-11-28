use amethyst::ecs::prelude::*;

use ::components::{
  Age,
  Family,
  Matriarch,
};

///Promotes the oldest creep if there is no matriarch.
#[derive(Default)]
pub struct MatriarchPromote;

impl<'s> System<'s> for MatriarchPromote {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Age>,
    ReadStorage<'s, Family>,
    WriteStorage<'s, Matriarch>,
  );

  fn run(&mut self, (entities, age, family, mut matriarchs): Self::SystemData) {
    for (e, _) in (&entities, &matriarchs).join() {
      if entities.is_alive(e) {
        //If there is already an alive matriarch, nothing to do
        return;
      }
    }

    let mut eldest = None;
    let mut b_age = -1.0;

    for (e, _, a) in (&entities, &family, &age).join() {
      if !entities.is_alive(e) {
        continue;
      }
      if a.seconds > b_age {
        eldest = Some(e);
        b_age = a.seconds;
      }
    }

    if let Some(eldest) = eldest {
      matriarchs
        .insert(eldest, Matriarch {
          age_when_promoted: b_age,
        })
        .expect("Failed inserting component");
    }
  }
}