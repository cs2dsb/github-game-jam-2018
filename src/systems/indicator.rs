use amethyst::{
  core::{
    transform::components::Transform,
    cgmath::{
      Vector3,
      Quaternion,
      Euler,
      Deg,
    },
  },
  ecs::prelude::*,
  renderer::Shape,
};

use ::{
  components::{
    Indicator as IndicatorComponent,
    Matriarch,
    Shape as ShapeComponent,
    Color,
  },
};

///Adds indicators to matriarchs, moves existing indicators to track their target and removes indicators when the target dies.
#[derive(Default)]
pub struct Indicator;

impl<'s> System<'s> for Indicator {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, IndicatorComponent>,
    WriteStorage<'s, Transform>,
    Read<'s, LazyUpdate>,
  );

  fn run(&mut self, (entities, matriarchs, indicators, mut transforms, updater): Self::SystemData) {
    let mut need_indicators = Vec::new();
    for (e, _) in (&entities, &matriarchs).join() {
      if entities.is_alive(e) {
        need_indicators.push(e);
      }
    }

    for (e, i) in (&entities, &indicators).join() {
      need_indicators.retain( |&n| n != i.target );

      if !entities.is_alive(i.target) {
        //destroy indicators who's target has died
        entities
          .delete(e)
          .expect("Failed to delete entity");
      } else {
        //move indicators to the targets location
        let mut new_translation = None;
        if let Some(t) = transforms.get(i.target) {
          new_translation = Some(t.translation);
        }
        if let (Some(new_translation), Some(indicator_transform)) = (new_translation, transforms.get_mut(e)) {
          indicator_transform.translation = new_translation + i.offset;
        }
      }

    }

    //add indicators to matriarchs
    for n in need_indicators {
      let indicator = IndicatorComponent {
        target: n,
        offset: Vector3::new(0.0, 10.0, 0.0),
      };
      let mut transform = transforms.get(n).map_or(Transform::default(), |v| v.clone());
      transform.rotation = Quaternion::from(Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(-90.0) })
                         * Quaternion::from(Euler { x: Deg(0.0), y: Deg(90.0), z: Deg(0.0) });
      let shape = ShapeComponent {
        shape: Shape::Cone(10),
        scale: (4.0, 4.0, 4.0),
      };
      let color = Color::new(0.8, 0.2, 0.2, 1.0);
      updater
        .create_entity(&entities)
        .with(indicator)
        .with(transform)
        .with(shape)
        .with(color)
        .build();
    }
  }
}