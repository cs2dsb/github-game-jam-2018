use amethyst::shrev::EventChannel;

#[derive(Debug)]
pub enum Command {
  DropCube,
  DropLift,
  DropDirectionChanger,
  KillMatriarch,
  //-1 to 1 based off the user input axis value
  Zoom(f32),
}

pub type CommandChannel = EventChannel<Command>;