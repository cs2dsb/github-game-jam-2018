use amethyst::shrev::EventChannel;

#[derive(Debug)]
pub enum Command {
  DropCube,
  DropLift,
  KillMatriarch,
}

pub type CommandChannel = EventChannel<Command>;