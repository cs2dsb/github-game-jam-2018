use amethyst::{
  core::bundle::{Result, SystemBundle},
  ecs::DispatcherBuilder,
};

use super::BasicVelocity;
use super::LogFps;
use super::CameraMovement;
use super::PhysicsStep;
use super::PhysicsVisualizer;
use super::Murder;
use super::Walker;
use super::PlayerInput;
use super::DropCube;
use super::DropLift;
use super::ShapeVisualizer;
use super::Spawner;
use super::DropDirectionChanger;
use super::PhysicsTransformUpdate;
use super::Exit;
use super::Remove;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
      builder.add(PhysicsStep::default(), "physics_step_system", &[]);
      builder.add(Exit::default(), "exit_system", &["physics_step_system"]);

      builder.add(Walker::default(), "walker_system", &[]);
      builder.add(LogFps::default(), "log_fps_system", &[]);
      builder.add(BasicVelocity::default(), "basic_velocity_system", &[]);
      builder.add(CameraMovement::default(), "camera_movement_system", &[]);
      builder.add(PhysicsVisualizer::default(), "physics_visualizer_system", &[]);
      builder.add(ShapeVisualizer::default(), "shape_visualizer_system", &[]);
      builder.add(PlayerInput::default(), "player_input_system", &[]);

      //Murdering needs to happen last to make sure other commands are executed on the
      //matriarch before it's destroyed
      builder.add(DropCube::default(), "drop_cube_system", &["player_input_system"]);
      builder.add(DropLift::default(), "drop_lift_system", &["player_input_system"]);
      builder.add(DropDirectionChanger::default(), "drop_direction_changer_system", &["player_input_system"]);
      builder.add(Spawner::default(), "spawner_system", &[]);
      //Depends on spawner_system so the spawner gets a chance to update the matriarch before it's potentially killed
      //Otherwise the matriarch can be marked for removal but no other promoted because it's still alive until maintain is called
      builder.add(Murder::default(), "murder_system", &[
        "player_input_system",
        "drop_cube_system",
        "drop_lift_system",
        "spawner_system",
        "drop_direction_changer_system",
      ]);

      builder.add(Remove::default(), "remove_system", &["exit_system", "murder_system"]);

      builder.add(PhysicsTransformUpdate::default(), "physics_transform_update_system", &["physics_step_system"]);

      //NOTE: builder.print_par_seq was very useful in working out why dependencies seemed to be reversed
      // in the murder/drop_cube systems. What was really happening was:
      //  1. drop, 2. player_input, 3. murder
      // So it looked like murder was running before drop but really it was the player input was getting
      // inserted at the wrong point.

      debug!("Bundle execution plan:\n{:?}", builder);
      Ok(())
    }
}
