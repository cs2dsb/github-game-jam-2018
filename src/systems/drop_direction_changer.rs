use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
  core::{
    transform::Transform,
    cgmath::{
      Vector2,
    },
  },
};

use ::{
  components::{
    Matriarch,
    Walker,
    Collider,
    ChangeDirection as ChangeDirectionComponent,
  },
  config::PhysicsConfig,
  resources::{
    Command,
    CommandChannel,
    PhysicsWorld,
    Sprites,
  },
};

///Drops a direction changer sensor on the matriarch. Also does the proximity checking between cd sensors and walkers.
//TODO: Split the proximity checking into another system.
#[derive(Default)]
pub struct DropDirectionChanger {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for DropDirectionChanger {
  type SystemData = (
    Entities<'s>,
    Read<'s, CommandChannel>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Transform>,
    WriteStorage<'s, Walker>,
    ReadStorage<'s, ChangeDirectionComponent>,
    ReadStorage<'s, Collider>,
    Write<'s, PhysicsWorld>,
    Read<'s, PhysicsConfig>,
    ReadExpect<'s, Sprites>,
    Read<'s, LazyUpdate>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities, commands, matriarchs, transforms, mut walkers, change_direction_components, colliders, mut physics_world, physics_config, sprites, updater): Self::SystemData) {
    let mut drop_direction_changer = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::DropDirectionChanger => drop_direction_changer = true,
        _ => {},
      }
    }

    if drop_direction_changer {
      for (e, _, t, w) in (&entities, &matriarchs, &transforms, &walkers).join() {
        if entities.is_alive(e) {
          debug!("Dropping direction changer on Matriarch {:?}", e);

          let direction = w.direction.reversed();
          //let z_rot = match direction {
          //  Direction::Right => 0.0,
          //  Direction::Left => 180.0,
          //};

          let sprite = sprites.change_direction.clone();

          let mut transform = t.clone();

          let sensor = physics_world.create_ground_box_sensor(
            &Vector2::new(transform.translation.x, transform.translation.y), //Pos
            &Vector2::new(physics_config.change_direction_width * 0.5, physics_config.change_direction_height * 0.5), //Size
            0.0);

          let change_direction = ChangeDirectionComponent {
            direction: direction,
          };

          updater
            .create_entity(&entities)
            .with(sprite)
            .with(transform)
            .with(sensor)
            .with(change_direction)
            .build();
        }
      }
    }

    //Go through fetching all sensors and checking if walkers are in proximity
    let mut changed = Vec::new();
    for (changer, sensor) in (&change_direction_components, &colliders).join() {
      //Go through all other colliders in it's proximity
      if let Some(proxs) = physics_world.get_proximity(&sensor.collider_handle) {
        for prox in proxs {
          if let Some(entity) = physics_world.get_entity_for_collider(prox) {
            if let Some(walker) = walkers.get_mut(entity) {
              if walker.direction != changer.direction {
                debug!("Changing direction of {:?}", entity);
                walker.direction = changer.direction;
                //We want to change the velocity of these but physics_world is already borrowed
                //not sure if there's a better way to do this...
                if let Some(body_handle) = physics_world.get_body_for_collider(prox) {
                  changed.push(*body_handle);
                }
              }
            }
          }
        }
      }
    }
    for c in changed {
      if let Some(body) = physics_world.world.rigid_body_mut(c) {
        let mut velocity = *body.velocity();
        velocity.linear.x = 0.0;
        velocity.angular = 0.0;
        body.set_velocity(velocity);
      }
    }
  }
}