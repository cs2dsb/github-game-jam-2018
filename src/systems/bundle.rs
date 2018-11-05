use amethyst::{
  core::bundle::{Result, SystemBundle},
  ecs::DispatcherBuilder,
};

use super::BasicVelocity;
use super::LogFps;
use super::PawnMovement;
use super::PhysicsStep;
use super::PhysicsVisualizer;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
      builder.add(LogFps, "log_fps_system", &["fps_counter_system"]);
      builder.add(BasicVelocity, "basic_velocity_system", &[]);
      builder.add(PawnMovement, "pawn_movement_system", &[]);
      builder.add(PhysicsStep, "physics_step_system", &[]);
      builder.add(PhysicsVisualizer, "physics_visualizer_system", &[]);
      Ok(())
    }
}
