use amethyst::{
  core::bundle::{Result, SystemBundle},
  ecs::DispatcherBuilder,
};

use super::BasicVelocity;
use super::LogFps;
use super::CameraMovement;
use super::PhysicsStep;
use super::PhysicsVisualizer;
use super::Family;
use super::Walker;
use super::PlayerInput;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
      builder.add(LogFps::default(), "log_fps_system", &["fps_counter_system"]);
      builder.add(BasicVelocity::default(), "basic_velocity_system", &[]);
      builder.add(CameraMovement::default(), "camera_movement_system", &[]);
      builder.add(PhysicsVisualizer::default(), "physics_visualizer_system", &[]);
      builder.add(Family::default(), "family_system", &[]);
      builder.add(Walker::default(), "walker_system", &[]);
      builder.add(PhysicsStep::default(), "physics_step_system", &["walker_system"]);
      builder.add(PlayerInput::default(), "player_input_system", &[]);
      Ok(())
    }
}
