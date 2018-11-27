use amethyst::{
  prelude::*,
  assets::{
    AssetStorage,
    Completion,
    Loader,
    ProgressCounter,
    Handle,
    RonFormat,
    Prefab,
  },
  renderer::{
    DebugLinesComponent,
    DebugLinesParams,
  },
  ui::{
    UiCreator,
    UiFinder,
    UiFormat,
    UiPrefab,
  },
};

use ::{
  config::{
    PhysicsConfig,
    CameraConfig,
  },
  resources::{
    PhysicsWorld,
    Sounds,
    Sprites,
  }
};

use super::{
  RunningState,
  RunningPrefabData,
};

///Loads required assets and makes sure everythin is ready before moving to the running state.
#[derive(Default)]
pub struct LoadingState {
  progress: ProgressCounter,
  running_ui_handle: Option<(Handle<UiPrefab>)>,
  running_prefab_handle: Option<(Handle<Prefab<RunningPrefabData>>)>,
  loader_complete: bool,
}

impl<'a, 'b> SimpleState<'a, 'b> for LoadingState {
  fn on_start(&mut self, data: StateData<GameData>) {
    info!("LoadingState.on_start");
    let world = data.world;

    add_debug_lines(world);
    configure_physics(world);
    add_loading_ui(world, &mut self.progress);
    self.load_running_prefab(world);
    self.load_running_ui(world);
    self.load_sounds(world);
    self.load_sprites(world);
  }
  fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
    if !self.loader_complete {
      match self.progress.complete() {
        Completion::Loading => {},
        Completion::Complete => self.loader_complete = true,
        Completion::Failed => {
          error!("Failed to load assets, exiting");
          return Trans::Quit
        },
      }
    }
    if self.loader_complete {
      let running_ui = self
                          .running_ui_handle
                          .take()
                          .expect("LoadingState.running_ui_handle was None after loading was finished");
      let running_prefab = self
                          .running_prefab_handle
                          .take()
                          .expect("LoadingState.running_prefab_handle was None after loading was finished");

      return Trans::Switch(
        Box::new(
          RunningState::new(running_ui, running_prefab)
        )
      )
    }
    Trans::None
  }
  fn on_stop(&mut self, data: StateData<GameData>) {
    if let Some(e) = data.world.exec(|finder: UiFinder| finder.find("loading")) {
      data.world
        .delete_entity(e)
        .expect("Failed to remove loading ui");
    }
  }
}

impl LoadingState {
  fn load_running_prefab(&mut self, world: &mut World) {
    let loader = world.read_resource::<Loader>();
    let prefab_storage = world.read_resource::<AssetStorage<Prefab<RunningPrefabData>>>();
    let prefab_handle = loader.load(
      "prefab/running.ron",
      RonFormat,
      Default::default(),
      &mut self.progress,
      &prefab_storage,
    );

    self.running_prefab_handle = Some(prefab_handle);
  }

  fn load_running_ui(&mut self, world: &mut World) {
    let loader = world.read_resource::<Loader>();
    let ui_storage = world.read_resource::<AssetStorage<UiPrefab>>();
    let ui_handle = loader.load(
      "ui/running.ron",
      UiFormat,
      Default::default(),
      &mut self.progress,
      &ui_storage,
    );

    self.running_ui_handle = Some(ui_handle);
  }

  fn load_sounds(&mut self, world: &mut World) {
    let sounds = Sounds::new(
      &world.read_resource(), //Loader
      &world.read_resource(), //AssetStorage<Source>
      &mut self.progress,
      &world.read_resource(), //SoundConfig
    );
    world.add_resource(sounds);
  }

  fn load_sprites(&mut self, world: &mut World) {
    let sprites = Sprites::new(
      world,
      &mut self.progress,
    );
    world.add_resource(sprites);
  }
}

fn add_loading_ui(world: &mut World, progress: &mut ProgressCounter) {
  //TODO: Currently if a sub resource (e.g. a font) fails to load the error isn't passed to our ProgressCounter.
  //      See amethyst_assets::prefab::Prefab::trigger_sub_loading for why (spoiler, it creates a default progress counter)
  //      Fixing it would require quite a lot of changes so I'm skipping it for now
  world.exec(|mut creator: UiCreator| {
    creator.create("ui/loading.ron", progress);
  });
}

fn configure_physics(world: &mut World) {
  let physics_config = world.read_resource::<PhysicsConfig>();
  let mut physics_world = world.write_resource::<PhysicsWorld>();
  physics_world.set_gravity(physics_config.gravity);
}

fn add_debug_lines(world: &mut World) {
  let gridline_width = world.read_resource::<CameraConfig>().gridline_width;
  if gridline_width <= 0.0 {
    return;
  }

  {
    let mut params = world.write_resource::<DebugLinesParams>();
    params.line_width = gridline_width;
  }
  let mut debug_lines_component = DebugLinesComponent::new().with_capacity(600);
  for n in 0..100 {
    let nf = n as f32;
    debug_lines_component.add_line(
      [nf * 64.0, 0.0, 0.0].into(),
      [nf * 64.0, 100.0 * 64.0, 0.0].into(),
      [0.5, 0.85, 0.1, 1.0].into(),
    );

    debug_lines_component.add_line(
      [0.0, 0.0, nf * 64.0].into(),
      [0.0, 100.0 * 64.0, nf * 64.0].into(),
      [0.5, 0.85, 0.1, 1.0].into(),
    );

    debug_lines_component.add_line(
      [0.0, nf * 64.0, 0.0].into(),
      [100.0 * 64.0, nf * 64.0, 0.0].into(),
      [1.0, 0.0, 0.23, 1.0].into(),
    );

    debug_lines_component.add_line(
      [0.0, 0.0, nf * 64.0].into(),
      [100.0 * 64.0, 0.0, nf * 64.0].into(),
      [1.0, 0.0, 0.23, 1.0].into(),
    );

    debug_lines_component.add_line(
      [nf * 64.0, 0.0, 0.0].into(),
      [nf * 64.0, 0.0, 100.0 * 64.0].into(),
      [0.2, 0.75, 0.93, 1.0].into(),
    );

    debug_lines_component.add_line(
      [0.0, nf * 64.0, 0.0].into(),
      [0.0, nf * 64.0, 100.0 * 64.0].into(),
      [0.2, 0.75, 0.93, 1.0].into(),
    );
  }

  world
    .create_entity()
    .with(debug_lines_component)
    .build();
}