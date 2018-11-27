use amethyst::{
  ecs::prelude::*,

  assets::{
    AssetStorage,
    Loader,
    ProgressCounter,
  },

  renderer::{
    MaterialTextureSet,
    PngFormat,
    Sprite,
    SpriteRender,
    SpriteSheet,
    TextureCoordinates,
    TextureMetadata,
    Texture,
  },
  utils::application_root_dir,
};

use ::config::SpritesConfig;

const TEXTURE_PATH: &'static str = "assets/texture";
const SPRITESHEET_FILE: &'static str = "spritesheet.png";

const LIFT_SPRITE_NAME: &'static str = "lift";
const CHANGE_DIRECTION_SPRITE_NAME: &'static str = "change_direction";

// `texture_id` is a application defined ID given to the texture to store in the `World`.
// This is needed to link the texture to the sprite_sheet.
const TEXTURE_ID: u64 = 0;

///Resource that contains templates for the sprites the game uses.
pub struct Sprites {
  pub lift: SpriteRender,
  pub change_direction: SpriteRender,
}

fn find_and_load_named_sprite(name: &str, sprites_config: &SpritesConfig) -> Sprite {
  let spritesheet_height = sprites_config.sheet_height as f32;
  let spritesheet_width = sprites_config.sheet_width as f32;
  for sc in &sprites_config.sprites {
    if sc.name == name {
      let w = sc.sheet_width as f32;
      let h = sc.sheet_height as f32;
      let x = sc.sheet_x as f32;
      let y = sc.sheet_y as f32;

      let tex_coords = TextureCoordinates {
        left: x / spritesheet_width,
        right: (x + w) / spritesheet_width,
        bottom: 1.0 - y / spritesheet_height,
        top: 1.0 - (y + h) / spritesheet_height,
      };

      let w = sc.scaled_width as f32;
      let h = sc.scaled_height as f32;
      let x = sc.scaled_x as f32;
      let y = sc.scaled_y as f32;

      return Sprite {
        width: w,
        height: h,
        offsets: [x, y],
        tex_coords: tex_coords,
      };
    }
  }
  panic!("Failed to find sprite named {}", name);
}

impl Sprites {
  pub fn new(world: &mut World, progress: &mut ProgressCounter) -> Self {
    {
      let root_dir = application_root_dir();

      let loader = world.read_resource::<Loader>();
      let storage = world.read_resource::<AssetStorage<Texture>>();
      let texture_handle = loader.load(
        format!("{}/{}/{}", root_dir, TEXTURE_PATH, SPRITESHEET_FILE),
        PngFormat,
        TextureMetadata::srgb_scale(),
        progress,
        &storage,
      );

      let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
      material_texture_set.insert(TEXTURE_ID, texture_handle);
    }

    let (lift_sprite, cd_sprite) = {
      let sprites_config = world.read_resource::<SpritesConfig>();
      let lift_sprite = find_and_load_named_sprite(LIFT_SPRITE_NAME, &sprites_config);
      let cd_sprite = find_and_load_named_sprite(CHANGE_DIRECTION_SPRITE_NAME, &sprites_config);
      (lift_sprite, cd_sprite)
    };

    let sprite_sheet = SpriteSheet {
      texture_id: TEXTURE_ID,
      sprites: vec![lift_sprite, cd_sprite],
    };

    let sprite_count = sprite_sheet.sprites.len();

    let sprite_sheet_handle = {
      let loader = world.read_resource::<Loader>();
      let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
      loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
    };

    let lift_render = SpriteRender {
      sprite_sheet: sprite_sheet_handle.clone(),
      sprite_number: 0,
      flip_horizontal: false,
      flip_vertical: true, //TODO: This shouldn't be necessary, mistake in tex_coords maybe?
    };

    let cd_render = SpriteRender {
      sprite_sheet: sprite_sheet_handle.clone(),
      sprite_number: 1,
      flip_horizontal: false,
      flip_vertical: false,
    };


    //These are just to check for typos in sprite_number values above
    //TODO: a better way of registering sprites and tracking their number
    assert!(lift_render.sprite_number < sprite_count);
    assert!(cd_render.sprite_number < sprite_count);

    Sprites {
      lift: lift_render,
      change_direction: cd_render,
    }
  }
}