use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
};

use ::{
  components::{
    Matriarch,
    Age,
  },
  resources::{
    Command,
    CommandChannel,
    SpawnStats,
  },
  config::PhysicsConfig,
};

///Kills the current matriarch.
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
    Read<'s, PhysicsConfig>,
    ReadStorage<'s, Age>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities,  matriarchs, mut spawn_stats, commands, physics_config, ages): Self::SystemData) {
    let mut murder = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::KillMatriarch => murder = true,
        _ => {},
      }
    }

    if murder {
      for (e, m, a) in (&entities, &matriarchs, &ages).join() {
        if entities.is_alive(e) {

          //This test is to discard commands that were likely intended for a matriarch that just died
          if (a.seconds - m.age_when_promoted) < physics_config.matriarch_grace_period {
            continue;
          }

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