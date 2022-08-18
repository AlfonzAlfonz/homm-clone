use super::map::*;
use super::GameScale;
use crate::world::MapClickEvent;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup).add_system(move_player);
  }
}

#[derive(Component)]
pub struct Player {
  pub x: isize,
  pub y: isize,
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
) {
  let texture_handle = asset_server.load("hero.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 51.0), 5, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands
    .spawn()
    .insert(Player { x: 0, y: 0 })
    .insert_bundle(SpriteSheetBundle {
      transform: Transform {
        translation: Vec3::new(0., 20., 2.),
        ..default()
      },
      texture_atlas: texture_atlas_handle,
      ..default()
    });
}

fn move_player(
  mut events: EventReader<MapClickEvent>,
  mut query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Player), With<Player>>,
) {
  let r = query.single_mut();
  let mut transform = r.0;
  let mut sprites = r.1;
  let mut player = r.2;

  for e in events.iter() {
    player.x = e.x as isize;
    player.y = e.y as isize;
  }

  let dx = (player.x as f32 * TILE_SIZE as f32 - transform.translation.x) + 14.;
  let dy = (player.y as f32 * TILE_SIZE as f32 - transform.translation.y) + 32.;

  if dx != 0. && dy > 0. {
    sprites.index = 4;
  } else if dx != 0. && dy < 0. {
    sprites.index = 1;
  } else if dx == 0. && dy > 0. {
    sprites.index = 3;
  } else if dx == 0. && dy < 0. {
    sprites.index = 2;
  } else if dx != 0. && dy == 0. {
    sprites.index = 0;
  }

  if dx < 0. {
    sprites.flip_x = true;
  } else if dx > 0. {
    sprites.flip_x = false;
  }

  transform.translation.x += if dx > 2. {
    2.
  } else if dx < -2. {
    -2.
  } else {
    dx
  };
  transform.translation.y += if dy > 2. {
    2.
  } else if dy < -2. {
    -2.
  } else {
    dy
  };
}
