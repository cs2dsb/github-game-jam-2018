use amethyst::{
  GameDataBuilder,
  renderer::{
    RenderBundle,
    Pipeline,
    DrawSprite,
    DrawShaded,
    ColorMask,
    ALPHA,
    Stage,
    DisplayConfig,
    DrawDebugLines,
    PosColorNorm,
    PosNormTex,
  },
  ui::{
    DrawUi,
    UiBundle,
  },
};

//Configures render passes and registeres rendering related systems
pub fn configure_rendering<'a, 'b>(builder: GameDataBuilder<'a, 'b>, display_config: DisplayConfig) -> Result<GameDataBuilder<'a, 'b>, amethyst::Error> {
  let pipe = Pipeline::build().with_stage(
    Stage::with_backbuffer()
      .clear_target([0.05, 0.05, 0.05, 1.0], 1.0)
      .with_pass(DrawShaded::<PosNormTex>::new())
      .with_pass(DrawUi::new())
      .with_pass(DrawSprite::new()
        .with_transparency(ColorMask::all(), ALPHA, None))
      .with_pass(DrawDebugLines::<PosColorNorm>::new())
  );

  builder.with_bundle(RenderBundle::new(pipe, Some(display_config))
    .with_sprite_sheet_processor())?
    .with_bundle(UiBundle::<String, String>::new())
}