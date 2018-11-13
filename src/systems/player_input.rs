use std::collections::HashSet;

use amethyst::{
  ecs::prelude::*,
  input::InputHandler,
};

use ::resources::{
  Command,
  CommandChannel,
};

#[derive(Default)]
pub struct PlayerInput {
  down_actions: HashSet<String>,
}

impl<'s> System<'s> for PlayerInput {
  type SystemData = (
    Read<'s, InputHandler<String, String>>,
    Write<'s, CommandChannel>,
  );

  fn run(&mut self, (input, mut commands): Self::SystemData) {
    for action in input.bindings.actions() {
      let was_down = self.down_actions.contains(&action);
      let is_down = input.action_is_down(&action).unwrap_or(false);

      let pressed = !was_down && is_down;
      let released = was_down && !is_down;

      if released {
        self.down_actions.remove(&action);
      } else if pressed {
        match action.as_ref() {
          "one" => commands.single_write(Command::DropCube),
          "two" => commands.single_write(Command::DropLift),
          "three" => commands.single_write(Command::DropDirectionChanger),
          o => debug!("Unhandled input action: {:?}", o),
        }
        //All other actions also kill the matriach (for now)
        commands.single_write(Command::KillMatriarch);
        self.down_actions.insert(action);
      }
    }

    for axis in input.bindings.axes() {
      let value = input.axis_value(&axis).unwrap_or(0.0);
      if value != 0.0 {
        match axis.as_ref() {
          "move_z" => commands.single_write(Command::Zoom(value as f32)),
          o => debug!("Unhandled input axis {} value: {}", o, value),
        }
      }
    }
  }
}