use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
  core::{
    cgmath::Vector2,
    transform::Transform,
  },
};

use ::{
  components::{
    Matriarch,
  },
  resources::{
    Command,
    CommandChannel,
    PhysicsWorld,
  },
};


#[derive(Default)]
pub struct DropCube {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for DropCube {
  type SystemData = (
    Entities<'s>,
    Read<'s, CommandChannel>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Transform>,
    Write<'s, PhysicsWorld>,
    Read<'s, LazyUpdate>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities, commands, matriarchs, transforms, mut physics_world, updater): Self::SystemData) {
    let mut drop_cube = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::DropCube => drop_cube = true,
        _ => {},
      }
    }

    if drop_cube {
      for (e, _, t) in (&entities, &matriarchs, &transforms).join() {
        if entities.is_alive(e) {
          debug!("Dropping cube on Matriarch {:?}", e);

          let collider = physics_world.create_rigid_body_with_box_collider(
            &Vector2::new(t.translation.x, t.translation.y),
            &Vector2::new(40.0, 40.0),
            0.0);

          updater
            .create_entity(&entities)
            .with(collider)
            .build();
        }
      }
    }
  }
}