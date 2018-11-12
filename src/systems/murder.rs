use amethyst::{
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
    SpawnStats,
  },
};


pub struct Murder {
  command_reader: Option<ReaderId<Command>>,
}

impl Default for Murder {
  fn default() -> Self {
    Self {
      command_reader: None,
    }
  }
}

impl<'s> System<'s> for Murder {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, FamilyComponent>,
    ReadStorage<'s, Matriarch>,
    Write<'s, SpawnStats>,
    Read<'s, LazyUpdate>,
    Read<'s, CommandChannel>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities, family_components, matriarchs, mut spawn_stats, updater, commands): Self::SystemData) {
    let mut murder = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::KillMatriarch => murder = true,
        _ => {},
      }
    }

    if murder {
      for (e, _) in (&entities, &matriarchs).join() {
        if entities.is_alive(e) {
          info!("Murdering Matriarch {:?}", e);
          spawn_stats.killed += 1;
          let mut next_matriarch = None;
          if let Some(fam) = family_components.get(e) {
            next_matriarch = fam.next;
          }
          if let Some(next_matriarch) = next_matriarch {
            info!("Promoted {:?} to Matriarch", next_matriarch);
            updater.insert(next_matriarch, Matriarch);
          }
          entities
            .delete(e)
            .expect("Failed to delete entity");
        }
      }
    }
  }
}