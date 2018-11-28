use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
  assets::AssetStorage,
  audio::{
    Source,
    output::Output,
  },
};

use ::{
  components::{
    Spawner
  },
  resources::{
    Command,
    CommandChannel,
    Sounds,
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
    WriteStorage<'s, Spawner>,
    ReadExpect<'s, Sounds>,
    Read<'s, AssetStorage<Source>>,
    Option<Read<'s, Output>>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (commands, mut spawners, sounds, source_storage, output): Self::SystemData) {
    let mut exodus = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::Exodus => exodus = true,
        _ => {},
      }
    }

    if exodus {
      if let Some(output) = &output {
        sounds.play_exodus(&source_storage, output);
      }
      for s in (&mut spawners).join() {
        s.exodus = true;
      }
    }
  }
}