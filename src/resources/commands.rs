use amethyst::shrev::EventChannel;

#[derive(Debug)]
pub enum Command {
  DropCube,
  DropLift,
  DropDirectionChanger,
  DropRam,
  KillMatriarch,
  //-1 to 1 based off the user input axis value
  Zoom(f32),
  ReloadLevels,
  NextLevel,
  RestartLevel,
  PreviousLevel,
}

pub fn also_kills(cmd: &Command) -> bool {
  match cmd {
    &Command::DropCube => true,
    &Command::DropLift => true,
    &Command::DropDirectionChanger => true,
    &Command::DropRam => true,
    &Command::KillMatriarch => false,
    &Command::Zoom(_) => false,
    &Command::ReloadLevels => false,
    &Command::NextLevel => false,
    &Command::RestartLevel => false,
    &Command::PreviousLevel => false,
  }
}

pub type CommandChannel = EventChannel<Command>;