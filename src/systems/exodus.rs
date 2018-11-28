use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
};

use ::{
  components::{
    Spawner
  },
  config::SpawnerConfig,
  resources::{
    Command,
    CommandChannel,
  },
};

///Causes all spawners to output at the maximum rate
#[derive(Default)]
pub struct Exodus {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for Exodus {
  type SystemData = (
    Read<'s, CommandChannel>,
    Read<'s, SpawnerConfig>,
    WriteStorage<'s, Spawner>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (commands, spawner_config, mut spawners): Self::SystemData) {
    let mut exodus = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::Exodus => exodus = true,
        _ => {},
      }
    }

    if exodus {
      let min_freq = spawner_config.frequency_min;
      for s in (&mut spawners).join() {
        s.frequency = min_freq;
        s.elapsed = s.frequency * (s.spawn_count - 1) as f32;
      }
    }
  }
}