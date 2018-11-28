use std::collections::HashSet;

use amethyst::{
  ecs::prelude::*,
  input::InputHandler,
};

use ::resources::{
  Command,
  CommandChannel,
  Sounds,
  also_kills,
};

///Checks the state of player input and sends commands for other systems to react to.
#[derive(Default)]
pub struct PlayerInput {
  down_actions: HashSet<String>,
}

impl<'s> System<'s> for PlayerInput {
  type SystemData = (
    Read<'s, InputHandler<String, String>>,
    Write<'s, CommandChannel>,
    WriteExpect<'s, Sounds>,
  );

  fn run(&mut self, (input, mut commands, mut sounds): Self::SystemData) {
    for action in input.bindings.actions() {
      let was_down = self.down_actions.contains(&action);
      let is_down = input.action_is_down(&action).unwrap_or(false);

      let pressed = !was_down && is_down;
      let released = was_down && !is_down;

      if released {
        self.down_actions.remove(&action);
      } else if pressed {
        let cmd = match action.as_ref() {
          "drop_cube" => Some(Command::DropCube),
          "drop_lift" => Some(Command::DropLift),
          "drop_direction_changer" => Some(Command::DropDirectionChanger),
          "reload_levels" => Some(Command::ReloadLevels),
          "next_level" => Some(Command::NextLevel),
          "prev_level" => Some(Command::PreviousLevel),
          "restart_level" => Some(Command::RestartLevel),
          "ram" => Some(Command::DropRam),
          "exodus" => Some(Command::Exodus),
          o => {
            debug!("Unhandled input action: {:?}", o);
            None
          },
        };
        if let Some(cmd) = cmd {
          let kill = also_kills(&cmd);
          commands.single_write(cmd);
          if kill {
            commands.single_write(Command::KillMatriarch);
          }
        }
        self.down_actions.insert(action);
      }
    }

    for axis in input.bindings.axes() {
      let value = input.axis_value(&axis).unwrap_or(0.0);
      if value != 0.0 {
        match axis.as_ref() {
          "move_z" => commands.single_write(Command::Zoom(value as f32)),
          "volume" => {
            let v = sounds.volume + 0.01 * value as f32;
            sounds.volume = v.min(1.0).max(0.0);
          },
          o => debug!("Unhandled input axis {} value: {}", o, value),
        }
      }
    }
  }
}