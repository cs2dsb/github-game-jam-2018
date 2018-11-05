use amethyst::{
  ecs::{
    Component,
    storage::{
      BTreeStorage,
    },
  },
};

mod basic_velocity;
mod physics;

pub use self::basic_velocity::BasicVelocity;
pub use self::physics::*;

//TODO: I've just used BTreeStorage for all of these as the specs book suggests it's ok as a general default.
//  Think about using more appropriate storages at some point.
impl Component for BasicVelocity {
  type Storage = BTreeStorage<Self>;
}

impl Component for Collider {
  type Storage = BTreeStorage<Self>;
}