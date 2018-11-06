use amethyst::{
  core::{
    timing::Time,
  },
  ecs::prelude::*,
  shrev::ReaderId,
};

use ::{
  components::{
    Family as FamilyComponent,
    Matriarch,
  },
  resources::{
    Command,
    CommandChannel,
  },
};


pub struct Family {
  elapsed: f32,
  command_reader: Option<ReaderId<Command>>,
}

impl Default for Family {
  fn default() -> Self {
    Self {
      elapsed: 0.0,
      command_reader: None,
    }
  }
}

impl<'s> System<'s> for Family {
  type SystemData = (
    Read<'s, Time>,
    Entities<'s>,
    ReadStorage<'s, FamilyComponent>,
    ReadStorage<'s, Matriarch>,
    Read<'s, LazyUpdate>,
    Read<'s, CommandChannel>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (time, entities, family_components, matriarchs, updater, commands): Self::SystemData) {
    self.elapsed += time.delta_seconds();

    let mut murder = self.elapsed >= 5.0;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      #[allow(unreachable_patterns)]
      match command {
        Command::KillMatriarch => murder = true,
        _ => {},
      }
    }

    if murder {
      self.elapsed = 0.0;
      for (e, _) in (&entities, &matriarchs).join() {
        if entities.is_alive(e) {
          info!("Murdering Matriarch {:?}", e);
          let mut next_matriarch = None;
          if let Some(fam) = family_components.get(e) {
            next_matriarch = fam.next;
          }
          if let Some(next_matriarch) = next_matriarch {
            info!("Promoted {:?} to Matriarch", next_matriarch);
            updater.insert(next_matriarch, Matriarch);
          }
          entities.delete(e).expect("Failed to delete entity");
        }
      }
    }
  }
}