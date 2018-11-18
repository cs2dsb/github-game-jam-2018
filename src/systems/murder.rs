use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
};

use ::{
  components::{
    Matriarch,
  },
  resources::{
    Command,
    CommandChannel,
    SpawnStats,
  },
};

#[derive(Default)]
pub struct Murder {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for Murder {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Matriarch>,
    Write<'s, SpawnStats>,
    Read<'s, CommandChannel>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities,  matriarchs, mut spawn_stats, commands): Self::SystemData) {
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
          debug!("Murdering Matriarch {:?}", e);
          spawn_stats.killed += 1;

          entities
            .delete(e)
            .expect("Failed to delete entity");
        }
      }
    }
  }
}