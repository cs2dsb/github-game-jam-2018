///Finds colliders without meshes and creates meshes for them

use amethyst::{
  assets::{
    AssetStorage,
    Loader,
  },
  core::{
    transform::components::Transform,
    cgmath::{
      Vector3,
      Quaternion,
      Euler,
      Rad,
    },
  },
  ecs::prelude::*,
  renderer::{
    Material,
    MaterialDefaults,
    Mesh,
    MeshHandle,
    PosNormTex,
    Shape,
    Texture,
  },
};

use ncollide2d::shape as ncshape;
use nalgebra::{
  Point2 as naPoint2,
  Isometry2,
};
use random_color::RandomColor;

use ::{
  components::{
    Collider,
  },
  resources::{
    PhysicsWorld,
    SCALE_PIXELS_PER_METER,
  },
};

const Z_SIZE: f32 = 10.0;
const Z_POS: f32 = 20.0;

pub struct PhysicsVisualizer;

impl<'s> System<'s> for PhysicsVisualizer {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Collider>,
    WriteStorage<'s, Transform>,
    Read<'s, PhysicsWorld>,
    Read<'s, LazyUpdate>,
    ReadExpect<'s, MaterialDefaults>,
    ReadExpect<'s, Loader>,
    ReadExpect<'s, AssetStorage<Texture>>,
    ReadExpect<'s, AssetStorage<Mesh>>,
    ReadStorage<'s, MeshHandle>,
  );

  fn run(&mut self, (entities, colliders, mut transforms, physics_world, updater, material_defaults, loader, texture_storage, mesh_storage, meshes): Self::SystemData) {
    //Create meshes for colliders that don't have them
    for (entity, c, _) in (&entities, &colliders, !&meshes).join() {
      let collider = physics_world
        .world
        .collider(c.collider_handle)
        .expect("Failed to resolve collider handle to collider");

      //Material
      let color = RandomColor::new().to_rgb_array();
      let color = [
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
        1.0];
      let material = create_colour_material(
        &material_defaults,
        &texture_storage,
        &loader,
        color,
      );
      updater.insert(entity, material);

      //Mesh
      let mesh = {
        let shape = collider.shape().as_ref();
        let margin = collider.data().margin();
        if let Some(s) = shape.as_shape::<ncshape::Cuboid<f32>>() {
          let he = s.half_extents();
          let w = (he.x + margin) * SCALE_PIXELS_PER_METER;
          let h = (he.y + margin) * SCALE_PIXELS_PER_METER;
          let size = Vector3::new(w, h, Z_SIZE);
          let verts = Shape::Cube.generate_vertices::<Vec<PosNormTex>>(Some(size.into()));
          let mesh = loader.load_from_data(verts.into(), (), &mesh_storage);
          mesh
        } else {
          panic!("Unknown collider shape in PhysicsVisualizer");
        }
      };
      updater.insert(entity, mesh);

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

/// Creates a solid material of the specified colour.
fn create_colour_material(
  material_defaults: &MaterialDefaults,
  texture_storage: &AssetStorage<Texture>,
  loader: &Loader,
  colour: [f32; 4]
) -> Material {
  let albedo = loader.load_from_data(colour.into(), (), texture_storage);
  Material {
    albedo,
    ..material_defaults.0.clone()
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