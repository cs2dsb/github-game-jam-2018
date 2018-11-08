use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
  core::{
    transform::Transform,
    cgmath::{
      Quaternion,
      Euler,
      Deg,
    },
  },
  renderer::Shape,
};

use nphysics2d::{
  solver::IntegrationParameters,
  force_generator::ForceGenerator,
  object::{
    BodyHandle,
    BodySet
  },
  math::{
    Force,
    Velocity,
  },
};

use nalgebra::{
  Point2,
  Vector2 as naVector2,
};

use ::{
  components::{
    Matriarch,
    Family,
    Collider,
    Shape as ShapeComponent,
    ForceGenerator as ForceGeneratorComponent,
  },
  config::PhysicsConfig,
  resources::{
    Command,
    CommandChannel,
    PhysicsWorld,
    FSize,
    SCALE_METERS_PER_PIXEL,
  },
};

#[derive(Debug)]
struct LiftForce {
  bodies: Vec<BodyHandle>, //Bodies affected by lift
  width: FSize, //How wide the lift is
  height: FSize, //How high above the lift the effect cuts out
  center: Point2<FSize>, //Center of the lift
  force: Force<FSize>,
}

impl LiftForce {
  fn new(width: FSize, height: FSize, center: Point2<FSize>, force: Force<FSize>, bodies: Vec<BodyHandle>) -> Self {
    Self {
      bodies,
      width,
      height,
      center,
      force,
    }
  }
}

impl ForceGenerator<FSize> for LiftForce {
  fn apply(&mut self, _: &IntegrationParameters<FSize>, bodies: &mut BodySet<FSize>) -> bool {
    //Remove any that have been destroyed
    self.bodies.retain(|b| bodies.contains(*b));

    for handle in &self.bodies {
      let delta = bodies.body_part(*handle).center_of_mass() - self.center;
      if delta.x.abs() > (self.width / 2.0) || delta.y < -0.1 || delta.y > self.height {
        continue;
      }

      //Clear any existing velocity to make the effect of the lift repeatable
      if let Some(rigid_body) = bodies.rigid_body_mut(*handle) {
        rigid_body.set_velocity(Velocity::zero());
      }

      let mut part = bodies.body_part_mut(*handle);

      //let force = part.as_ref().inertia() * self.acceleration;
      // Apply the force.
      part.apply_force(&self.force);
    }

    // If `false` is returned, the physis world will remove
    // this force generator after this call.
    //self.bodies.len() > 0

    //There doesn't appear to be a way to check if a fg still exists in the world, the call
    //to fetch it just panics if it's been deleted so best not let that happen automatically...
    true
  }
}

pub struct DropLift {
  command_reader: Option<ReaderId<Command>>,
}

impl Default for DropLift {
  fn default() -> Self {
    Self {
      command_reader: None,
    }
  }
}

impl<'s> System<'s> for DropLift {
  type SystemData = (
    Entities<'s>,
    Read<'s, CommandChannel>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Family>,
    ReadStorage<'s, Collider>,
    ReadStorage<'s, ForceGeneratorComponent>,
    Write<'s, PhysicsWorld>,
    Read<'s, PhysicsConfig>,
    Read<'s, LazyUpdate>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities, commands, matriarchs, transforms, family_components, colliders, generators, mut physics_world, physics_config, updater): Self::SystemData) {
    let mut drop_lift = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::DropLift => drop_lift = true,
        _ => {},
      }
    }

    if drop_lift {
      for (e, _, t) in (&entities, &matriarchs, &transforms).join() {
        if entities.is_alive(e) {
          info!("Dropping lift on Matriarch {:?}", e);

          //Get all the bodies we want to affect with this lift
          let mut bodies = Vec::new();
          for (c, _) in (&colliders, &family_components).join() {
            bodies.push(c.body_handle);
          }

          let lift = LiftForce::new(
            physics_config.lift_width * SCALE_METERS_PER_PIXEL,
            physics_config.lift_height * SCALE_METERS_PER_PIXEL,
            Point2::new(t.translation.x * SCALE_METERS_PER_PIXEL, t.translation.y * SCALE_METERS_PER_PIXEL),
            Force::new(naVector2::new(physics_config.lift_force.x, physics_config.lift_force.y), 0.0),
            bodies);

          let fg = ForceGeneratorComponent {
            force_generator_handle: physics_world.world.add_force_generator(lift),
          };

          let shape = ShapeComponent {
            shape: Shape::Cone(10),
            scale: (2.0, 2.0, 2.0),
          };

          let mut transform = t.clone();
          transform.rotation = Quaternion::from(Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(90.0) })
                             * Quaternion::from(Euler { x: Deg(0.0), y: Deg(90.0), z: Deg(0.0) });

          updater
            .create_entity(&entities)
            .with(fg)
            .with(shape)
            .with(transform)
            .build();
        }
      }
    }

    //TODO: Move this to it's own system or inside spawner system
    //Update lifts
    for g in (&generators).join() {
      let mut fg = physics_world.world.force_generator_mut(g.force_generator_handle);
      if let Ok(lift) = fg.downcast_mut::<LiftForce>() {
        for (c, _) in (&colliders, &family_components).join() {
          if !lift.bodies.contains(&c.body_handle) {
            lift.bodies.push(c.body_handle);
          }
        }
      }
    }
  }
}