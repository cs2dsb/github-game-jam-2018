use std::collections::HashMap;

//TODO: This resource uses both cgmath and nalgebra. Amethyst is moving to nalgebra so this is a temporary cludge.
use amethyst::{
  core::cgmath::{
    Vector2 as CVector2,
  },
  ecs::prelude::*,
};
use nphysics2d::{
  object::{
    BodyHandle,
    Material,
    ColliderHandle,
  },
  world::World,
  volumetric::Volumetric,
};

use nalgebra::{
  Isometry2,
  Vector2,
};

use ncollide2d::{
  events::ContactEvent,
  query::Proximity,
  shape::{
    Cuboid,
    ShapeHandle,
  },
};

use ::components::{
  Collider,
};

pub type FSize = f32;

pub const SCALE_PIXELS_PER_METER: FSize = 64.0;
pub const SCALE_METERS_PER_PIXEL: FSize = 1.0 / SCALE_PIXELS_PER_METER;

//Plucked from arse
pub const MARGIN: FSize = 0.05 * SCALE_METERS_PER_PIXEL;

const TIMESTEP: f32 = 1.0/60.0;

///Resource that contains the nphysics world and manages collisions.
//It's a bit jankey but nphysics has a project underway to integrate it properly with specs
// which will replace this stuff.
pub struct PhysicsWorld {
  pub world: World<FSize>,
  time_accumulator: f32,
  timestep: f32,
  collider_entity_map: HashMap<ColliderHandle, Entity>,
  collider_body_map: HashMap<ColliderHandle, BodyHandle>,
  collider_contacts: HashMap<ColliderHandle, Vec<ColliderHandle>>,
  collider_proximity: HashMap<ColliderHandle, Vec<ColliderHandle>>,
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
      time_accumulator: 0.0,
      timestep: 0.0,
      collider_entity_map: HashMap::new(),
      collider_body_map: HashMap::new(),
      collider_contacts: HashMap::new(),
      collider_proximity: HashMap::new(),
    };
    s.set_gravity(-9.81);
    s.set_fixed_timestep(TIMESTEP);
    s
  }

  /*We aim to do one timestep per frame and render one frame behind.
    The amount of time left in the time_accumulator after a step represents how much
    we overshot our timestep by and can be used to lerp between the n and n+1 frames.
    This function gives that as a 0->1 value (>= 1 should be impossible).
  */
  pub fn get_alpha(&self) -> f32 {
    self.time_accumulator / self.timestep
  }

  pub fn register_entity(&mut self, entity: Entity, collider_handle: ColliderHandle) {
    debug!("Collider {:?} was associated with entity {:?}", collider_handle, entity);
    self.collider_entity_map.insert(collider_handle, entity);
  }

  pub fn get_entity_for_collider(&self, collider_handle: &ColliderHandle) -> Option<Entity> {
    self
      .collider_entity_map
      .get(collider_handle)
      .map(|e| e.clone())
  }

  pub fn get_body_for_collider(&self, collider_handle: &ColliderHandle) -> Option<&BodyHandle> {
    self.collider_body_map.get(collider_handle)
  }

  #[allow(dead_code)]
  pub fn get_contacts(&self, collider_handle: &ColliderHandle) -> Option<&[ColliderHandle]> {
    if let Some(contacts) = self.collider_contacts.get(collider_handle) {
      Some(&contacts)
    } else {
      None
    }
  }

  pub fn get_proximity(&self, collider_handle: &ColliderHandle) -> Option<&[ColliderHandle]> {
    if let Some(contacts) = self.collider_proximity.get(collider_handle) {
      Some(&contacts)
    } else {
      None
    }
  }

  pub fn set_gravity(&mut self, gravity: f32) {
    self.world.set_gravity(Vector2::y() * gravity);
  }

  pub fn set_fixed_timestep(&mut self, timestep: f32) {
    self.world.set_timestep(timestep);
    self.timestep = timestep;
  }

  fn process_contacts(&mut self) {
    let world = &self.world;
    let collider_contacts = &mut self.collider_contacts;
    for c in world.contact_events() {
      match c {
        ContactEvent::Started(c1, c2) => {
          add_contact(collider_contacts, c1, c2);
          add_contact(collider_contacts, c2, c1);
        },
        ContactEvent::Stopped(c1, c2) => {
          remove_contact(collider_contacts, c1, c2);
          remove_contact(collider_contacts, c2, c1);
        },
      }
    }
  }

  fn process_proximity(&mut self) {
    let world = &self.world;
    let collider_proximity = &mut self.collider_proximity;
    for c in world.proximity_events() {
      match c.new_status {
        Proximity::WithinMargin | Proximity::Intersecting => {
          add_contact(collider_proximity, &c.collider1, &c.collider2);
          add_contact(collider_proximity, &c.collider2, &c.collider1);
          debug!("Adding proximity: {:?}, {:?}", c.collider1, c.collider2);
        },
        Proximity::Disjoint => {
          remove_contact(collider_proximity, &c.collider1, &c.collider2);
          remove_contact(collider_proximity, &c.collider2, &c.collider1);
        },
      }
    }
  }

  fn do_step(&mut self) {
    self.world.step();
    self.process_contacts();
    self.process_proximity();
  }

  ///Adds time to the physics world, doesn't perform any steps
  pub fn add_time(&mut self, delta: f32) {
    self.time_accumulator += delta;
  }

  ///Steps the simulation if there is enough time in the accumulator
  pub fn step(&mut self) -> bool {
    if self.time_accumulator >= self.timestep {
      self.time_accumulator -= self.timestep;
      self.do_step();
      true
    } else {
      false
    }
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
    debug!("Created (ground) collider: {:?}", collider_handle);

    let body_handle = BodyHandle::ground();
    self.collider_body_map.insert(collider_handle, body_handle);

    Collider::new(body_handle, collider_handle)
  }

  pub fn create_ground_box_sensor(&mut self, pos: &CVector2<FSize>, size: &CVector2<FSize>, rotation: FSize) -> Collider {
    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
      //These are half extents
      size.x * 0.5 * SCALE_METERS_PER_PIXEL, //Note no margin
      size.y * 0.5 * SCALE_METERS_PER_PIXEL,
    )));
    let to_parent = Isometry2::new(Vector2::new(
      pos.x * SCALE_METERS_PER_PIXEL,
      pos.y * SCALE_METERS_PER_PIXEL),
      -rotation, //Rotation is not the way you'd think, presumably because this is to_parent not from_parent
    );
    let collider_handle = self.world.add_sensor(
      shape,
      BodyHandle::ground(),
      to_parent,
    );
    debug!("Created (ground) sensor: {:?}", collider_handle);

    let body_handle = BodyHandle::ground();
    self.collider_body_map.insert(collider_handle, body_handle);

    Collider::new(body_handle, collider_handle)
  }

  pub fn create_rigid_body_with_box_collider(&mut self, pos: &CVector2<FSize>, size: &CVector2<FSize>, rotation: FSize) -> Collider {
    self.create_rigid_body_with_box_collider_with_density(pos, size, rotation, 1.0)
  }

  pub fn create_rigid_body_with_box_collider_with_density(&mut self, pos: &CVector2<FSize>, size: &CVector2<FSize>, rotation: FSize, density: FSize) -> Collider {
    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
      //These are half extents
      size.x * 0.5 * SCALE_METERS_PER_PIXEL - MARGIN,
      size.y * 0.5 * SCALE_METERS_PER_PIXEL - MARGIN,
    )));
    let to_parent = Isometry2::identity();
    let pos = Isometry2::new(Vector2::new(
      pos.x * SCALE_METERS_PER_PIXEL,
      pos.y * SCALE_METERS_PER_PIXEL),
      -rotation, //Rotation is not the way you'd think, presumably because this is to_parent not from_parent
    );

    let body_handle = self.world.add_rigid_body(pos, shape.inertia(density), shape.center_of_mass());
    debug!("Created body: {:?}", body_handle);

    let collider_handle = self.world.add_collider(
      MARGIN,
      shape,
      body_handle,
      to_parent,
      Material::default(),
    );
    debug!("Created collider: {:?}", collider_handle);

    self.collider_body_map.insert(collider_handle, body_handle);

    Collider::new(body_handle, collider_handle)
  }

  ///Destroy a collider (also destroys the body if no colliders remain... this may not be what you want in all cases but it's convenient for now)
  pub fn destroy_collider(&mut self, collider: Collider) {
    //Wake up any things this is touching before destroying it (bug in nphysics https://github.com/rustsim/nphysics/issues/154)
    if let Some(contacts) = self.collider_contacts.remove(&collider.collider_handle) {
      for c in contacts {
        self.world.activate_body(
          *self.collider_body_map.get(&c).expect("Collider missing from body map"));

        //I thought you'd get Stopped events after the deletion but you don't
        remove_contact(&mut self.collider_contacts, &c, &collider.collider_handle);
      }
    }

    if let Some(prox) = self.collider_proximity.remove(&collider.collider_handle) {
      for p in prox {
        //I thought you'd get Disjoint events after the deletion but you don't
        remove_contact(&mut self.collider_proximity, &p, &collider.collider_handle);
      }
    }

    //Clean up the link to an entity
    self.collider_entity_map.remove(&collider.collider_handle);

    //Destroy the collider
    debug!("Destroying collider: {:?}", collider.collider_handle);
    self.world.remove_colliders(&[collider.collider_handle]);

    //If the body isn't the ground check to see if it's still used
    if !collider.body_handle.is_ground() {
      let mut found = false;
      for (_, bh) in &self.collider_body_map {
        if bh == &collider.body_handle {
          found = true;
          break;
        }
      }

      //If not, destroy the body too
      if !found {
        debug!("Destroying body: {:?}", collider.body_handle);
        self.world.remove_bodies(&[collider.body_handle]);
      }
    }
  }
}

fn add_contact(map: &mut HashMap<ColliderHandle, Vec<ColliderHandle>>, c1: &ColliderHandle, c2: &ColliderHandle) {
  map
    .entry(*c1)
    .or_insert(Vec::new())
    .push(*c2)
}

fn remove_contact(map: &mut HashMap<ColliderHandle, Vec<ColliderHandle>>, c1: &ColliderHandle, c2: &ColliderHandle) {
  let mut remove = false;
  if let Some(list) = map.get_mut(c1) {
    list.retain(|c| c != c2);
    if list.len() == 0 {
      remove = true;
    }
  }
  if remove {
    map.remove(c1);
  }
}