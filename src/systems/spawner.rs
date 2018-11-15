use amethyst::{
  core::{
    transform::components::Transform,
    timing::Time,
    cgmath::Vector2,
  },
  ecs::prelude::*,
};

use ncollide2d::world::CollisionGroups;

use ::{
  components::{
    Spawner as SpawnerComponent,
    Family,
    Walker,
    Matriarch,
  },
  resources::{
    PhysicsWorld,
    SpawnStats,
  },
};

pub struct Spawner {
  collision_groups: CollisionGroups,
}

impl Default for Spawner {
  fn default() -> Self {
    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[1]);

    //The whitelist is usually 1-29, we don't want to collide with ourselves so take 1 out
    let mut wl = Vec::new();
    wl.extend(2..=29);
    collision_groups.set_whitelist(&wl);

    Self {
      collision_groups,
    }
  }
}

impl<'s> System<'s> for Spawner {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Transform>,
    Read<'s, Time>,
    WriteStorage<'s, SpawnerComponent>,
    Write<'s, SpawnStats>,
    WriteStorage<'s, Family>,
    Write<'s, PhysicsWorld>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (entities, transforms, time, mut spawners, mut spawn_stats, mut family_components, mut physics_world, updater): Self::SystemData) {
    let delta = time.delta_seconds();

    //Increase elapsed time for all Spawners
    for (e, s, t) in (&entities, &mut spawners, &transforms).join() {
      s.elapsed += delta;
      if s.elapsed >= s.frequency {
        s.elapsed -= s.frequency;
        s.spawn_count += 1;

        spawn_stats.spawned += 1;

        let collider = {
          let collider = physics_world.create_rigid_body_with_box_collider(
            &Vector2::new(t.translation.x, t.translation.y),
            &s.spawn_size,
            0.0); //Rotation

          physics_world
            .world
            .collision_world_mut()
            .set_collision_groups(collider.collider_handle, self.collision_groups);

          collider
        };

        let new = updater
          .create_entity(&entities)
          .with(collider)
          .with(Family { next: None })
          .with(Walker::default())
          .build();

        debug!("Spawner ({:?}) spawned: {:?}", e, new);

        let mut need_matriarch = true;
        if let Some(last) = s.last_spawn {
          if entities.is_alive(last) {
            if let Some(fam) = family_components.get_mut(last) {
              fam.next = Some(new);
              need_matriarch = false;
            }
          }
        }

        if need_matriarch {
          debug!("Spawner ({:?}) promoted new spawn to Matriarch", e);
          updater.insert(new, Matriarch);
        }

        s.last_spawn = Some(new);
      }

      if s.spawn_count >= s.spawn_max {
        debug!("Spawner ({:?}) exhausted, deleting", e);
        entities
          .delete(e)
          .expect("Failed to delete entity");
      }
    }
  }
}