///Finds colliders without meshes and creates meshes for them

use amethyst::{
  core::{
    transform::components::Transform,
    cgmath::{
      Quaternion,
      Euler,
      Rad,
    },
  },
  ecs::prelude::*,
  renderer::Shape,
};

use ncollide2d::shape as ncshape;
use nalgebra::{
  Point2 as naPoint2,
  Isometry2,
};

use ::{
  components::{
    Collider,
    Shape as ShapeComponent,
  },
  resources::{
    PhysicsWorld,
    SCALE_PIXELS_PER_METER,
  },
};

const Z_SIZE: f32 = 10.0;
const Z_POS: f32 = 20.0;

#[derive(Default)]
pub struct PhysicsVisualizer;

impl<'s> System<'s> for PhysicsVisualizer {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Collider>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, ShapeComponent>,
    Read<'s, PhysicsWorld>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (entities, colliders, mut transforms, shapes, physics_world, updater): Self::SystemData) {
    //Create shapes for colliders that don't have them
    for (entity, c, _) in (&entities, &colliders, !&shapes).join() {
      let collider = physics_world
        .world
        .collider(c.collider_handle)
        .expect("Failed to resolve collider handle to collider");

      //Shape
      let shape = {
        let shape = collider.shape().as_ref();
        let margin = collider.data().margin();
        if let Some(s) = shape.as_shape::<ncshape::Cuboid<f32>>() {
          let he = s.half_extents();
          let w = (he.x + margin) * SCALE_PIXELS_PER_METER;
          let h = (he.y + margin) * SCALE_PIXELS_PER_METER;
          ShapeComponent {
            shape: Shape::Cube,
            scale: (w, h, Z_SIZE),
          }
        } else {
          panic!("Unknown collider shape in PhysicsVisualizer");
        }
      };
      updater.insert(entity, shape);

      //Transform
      let local_transform = {
        let mut local_transform = Transform::default();
        update_transform(&mut local_transform, &collider.position());
        local_transform
      };
      updater.insert(entity, local_transform);
    }

    //Update the transform based off the colliders position
    for (c, transform) in (&colliders, &mut transforms).join() {
      let collider = physics_world
        .world
        .collider(c.collider_handle)
        .expect("Failed to resolve collider handle to collider");

      update_transform(transform, &collider.position());
    }
  }
}

//TODO: This conversion should go away when amethyst moves to nalgebra instead of cgmath
fn update_transform(transform: &mut Transform, isometry: &Isometry2<f32>) {
  let point = naPoint2::from(isometry.translation.vector);
  transform.translation.x = point.x * SCALE_PIXELS_PER_METER;
  transform.translation.y = point.y * SCALE_PIXELS_PER_METER;
  transform.translation.z = Z_POS;
  transform.rotation = Quaternion::from(Euler {
    x: Rad(0.0),
    y: Rad(0.0),
    z: Rad(isometry.rotation.angle()),
  });
}