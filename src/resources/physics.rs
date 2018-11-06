use amethyst::core::cgmath::{
  Vector2 as CVector2,
};

use nphysics2d::{
  object::{
    BodyHandle,
    Material,
  },
  world::World,
  volumetric::Volumetric,
};

use nalgebra::{
  Isometry2,
  Vector2,
};

use ncollide2d::shape::{
  Cuboid,
  ShapeHandle,
};

use ::components::{
  Collider,
};

type FSize = f32;

pub const SCALE_PIXELS_PER_METER: FSize = 64.0;
pub const SCALE_METERS_PER_PIXEL: FSize = 1.0 / SCALE_PIXELS_PER_METER;

//Plucked from arse
pub const MARGIN: FSize = 0.01 * SCALE_METERS_PER_PIXEL;

pub struct PhysicsWorld {
  pub world: World<FSize>,
}

impl Default for PhysicsWorld {
  fn default() -> Self {
    Self::new()
  }
}

impl PhysicsWorld {
  fn new() -> Self {
    let mut s = Self {
      world: World::new(),
    };
    s.set_gravity(-9.81);
    s
  }

  pub fn set_gravity(&mut self, gravity: f32) {
    self.world.set_gravity(Vector2::y() * gravity);
  }

  pub fn step(&mut self, delta: FSize) {
    self.world.set_timestep(delta);
    self.world.step();
  }

  pub fn create_ground_box_collider(&mut self, pos: &CVector2<FSize>, size: &CVector2<FSize>, rotation: FSize) -> Collider {
    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
      //These are half extents
      size.x * 0.5 * SCALE_METERS_PER_PIXEL - MARGIN,
      size.y * 0.5 * SCALE_METERS_PER_PIXEL - MARGIN,
    )));
    let to_parent = Isometry2::new(Vector2::new(
      pos.x * SCALE_METERS_PER_PIXEL,
      pos.y * SCALE_METERS_PER_PIXEL),
      rotation,
    );
    let collider_handle = self.world.add_collider(
      MARGIN,
      shape,
      BodyHandle::ground(),
      to_parent,
      Material::default(),
    );

    Collider {
      body_handle: BodyHandle::ground(),
      collider_handle: collider_handle,
    }
  }

  pub fn create_rigid_body_with_box_collider(&mut self, pos: &CVector2<FSize>, size: &CVector2<FSize>, rotation: FSize) -> Collider {
    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
      //These are half extents
      size.x * 0.5 * SCALE_METERS_PER_PIXEL - MARGIN,
      size.y * 0.5 * SCALE_METERS_PER_PIXEL - MARGIN,
    )));
    let to_parent = Isometry2::identity();
    let pos = Isometry2::new(Vector2::new(
      pos.x * SCALE_METERS_PER_PIXEL,
      pos.y * SCALE_METERS_PER_PIXEL),
      rotation,
    );

    let body_handle = self.world.add_rigid_body(pos, shape.inertia(1.0), shape.center_of_mass());
    let collider_handle = self.world.add_collider(
      MARGIN,
      shape,
      body_handle,
      to_parent,
      Material::default(),
    );

    Collider {
      body_handle: body_handle,
      collider_handle: collider_handle,
    }
  }

  #[allow(dead_code)]
  pub fn destroy_collider(&mut self, collider: Collider, destroy_body_too: bool) {
    self.world.remove_colliders(&[collider.collider_handle]);
    if destroy_body_too && !collider.body_handle.is_ground() {
      self.world.remove_bodies(&[collider.body_handle]);
    }
  }
}