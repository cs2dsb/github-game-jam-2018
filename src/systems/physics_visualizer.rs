///Finds colliders without meshes and creates meshes for them

use amethyst::{
  ecs::prelude::*,
  renderer::Shape,
};

use ncollide2d::shape as ncshape;

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

#[derive(Default)]
pub struct PhysicsVisualizer;

impl<'s> System<'s> for PhysicsVisualizer {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Collider>,
    ReadStorage<'s, ShapeComponent>,
    Read<'s, PhysicsWorld>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (entities, colliders, shapes, physics_world, updater): Self::SystemData) {
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
    }
  }
}