use amethyst::shrev::EventChannel;

///Commands that various systems listen for. Most are user input.
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
  Exodus,
}

///Does the specified command also kill the matriarch?
//TODO: Bit jankey.
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
    &Command::Exodus => false,
  }
}

///This is the channel resource commands get sent to
pub type CommandChannel = EventChannel<Command>;