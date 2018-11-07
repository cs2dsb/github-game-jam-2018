use amethyst::shrev::EventChannel;

#[derive(Debug)]
pub enum Command {
  DropCube,
  KillMatriarch,
}

pub type CommandChannel = EventChannel<Command>;