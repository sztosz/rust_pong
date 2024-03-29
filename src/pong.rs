use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::transform::Transform,
  ecs::prelude::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Pong;

impl SimpleState for Pong {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    let sprite_sheet_handle = load_sprite_sheet(world);

    world.register::<Paddle>();
    initialize_paddles(world, sprite_sheet_handle);
    initialize_camera(world);
  }
}

pub enum Side {
  Left,
  Right,
}

pub struct Paddle {
  pub side: Side,
  pub height: f32,
  pub width: f32,
}

impl Paddle {
  fn new(side: Side) -> Paddle {
    Paddle {
      side,
      height: PADDLE_HEIGHT,
      width: PADDLE_WIDTH,
    }
  }
}

impl Component for Paddle {
  type Storage = DenseVecStorage<Self>;
}

fn initialize_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
    .with(transform)
    .build();
}

fn initialize_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
  let mut left_transform = Transform::default();
  let mut right_transform = Transform::default();

  let y = ARENA_HEIGHT / 2.0;
  left_transform.set_translation_xyz(PADDLE_WIDTH / 2.0, y, 0.0);
  right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH, y, 0.0);

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet.clone(),
    sprite_number: 0,
  };
  world
    .create_entity()
    .with(sprite_render.clone())
    .with(Paddle::new(Side::Right))
    .with(right_transform)
    .build();
  world
    .create_entity()
    .with(sprite_render.clone())
    .with(Paddle::new(Side::Left))
    .with(left_transform)
    .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/pong_spritesheet.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };
  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/pong_spritesheet.ron",
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
