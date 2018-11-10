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
      let is_down = {
        if let Some(down) = input.action_is_down(&action) {
          down
        } else {
          false
        }
      };

      let pressed = !was_down && is_down;
      let released = was_down && !is_down;

      if released {
        self.down_actions.remove(&action);
      } else if pressed {
        match action.as_ref() {
          "one" => commands.single_write(Command::DropCube),
          "two" => commands.single_write(Command::DropLift),
          "three" => commands.single_write(Command::DropDirectionChanger),
          _ => {},
        }
        //All other actions also kill the matriach (for now)
        commands.single_write(Command::KillMatriarch);
        self.down_actions.insert(action);
      }
    }
  }
}