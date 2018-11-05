use amethyst::{
  GameDataBuilder,
  utils::{
    fps_counter::FPSCounterSystem,
  },
  core::transform::TransformBundle,
};

mod basic_velocity;
mod bundle;
mod log_fps;
mod pawn_movement;
mod physics_step;
mod physics_visualizer;

pub use self::basic_velocity::*;
pub use self::bundle::*;
pub use self::log_fps::*;
pub use self::pawn_movement::*;
pub use self::physics_step::*;
pub use self::physics_visualizer::*;

//Not exactly sure how to structure this
//Want the function in systems so things like TransformBundle dependencies on my systems
//  aren't scattered about but it also probably doesn't make sense to configure everything
//  inside systems...
//Registers game systems and any core systems they depend on
pub fn register_systems<'a, 'b>(builder: GameDataBuilder<'a, 'b>) -> Result<GameDataBuilder<'a, 'b>, amethyst::Error> {
  builder.with(FPSCounterSystem, "fps_counter_system", &[])
         .with_bundle(GameBundle)?
         .with_bundle(TransformBundle::new())
      //.with_dep(&["paddle_system", "move_balls_system"]))?
}