use amethyst::{
  ecs::{
    Component,
    storage::{
      BTreeStorage,
      NullStorage,
      FlaggedStorage,
    },
  },
};

mod basic_velocity;
mod physics;
mod family;
mod walker;
mod shape;
mod spawner;
mod color;
mod exit;
mod remove;
mod indicator;
mod deadly_area;

pub use self::basic_velocity::*;
pub use self::physics::*;
pub use self::family::*;
pub use self::walker::*;
pub use self::shape::*;
pub use self::spawner::*;
pub use self::color::*;
pub use self::exit::*;
pub use self::remove::*;
pub use self::indicator::*;
pub use self::deadly_area::*;

//TODO: I've just used BTreeStorage for all of these as the specs book suggests it's ok as a general default.
//  Think about using more appropriate storages at some point.
impl Component for BasicVelocity {
  type Storage = BTreeStorage<Self>;
}

impl Component for Collider {
  type Storage = FlaggedStorage<Self, BTreeStorage<Self>>;
}

impl Component for Family {
  type Storage = BTreeStorage<Self>;
}

impl Component for Matriarch {
  type Storage = NullStorage<Self>;
}

impl Component for Walker {
  type Storage = BTreeStorage<Self>;
}

impl Component for Shape {
  type Storage = BTreeStorage<Self>;
}

impl Component for Spawner {
  type Storage = BTreeStorage<Self>;
}

impl Component for ForceGenerator {
  type Storage = BTreeStorage<Self>;
}

impl Component for ChangeDirection {
  type Storage = BTreeStorage<Self>;
}

impl Component for Color {
  type Storage = BTreeStorage<Self>;
}

impl Component for Exit {
  type Storage = NullStorage<Self>;
}

impl Component for Remove {
  type Storage = NullStorage<Self>;
}

impl Component for Indicator {
  type Storage = BTreeStorage<Self>;
}

impl Component for DeadlyArea {
  type Storage = NullStorage<Self>;
}