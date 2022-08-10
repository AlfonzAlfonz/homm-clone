use crate::map::TileClickEvent;
use crate::game::GameScale;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_system(move_player);
  }
}

#[derive(Component)]
pub struct Player {
  pub x: i32,
  pub y: i32
}

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
  scale: Res<GameScale>
) {
  let texture_handle = asset_server.load("hero.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 51.0), 5, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands
    .spawn()
    .insert(Player { x: 0, y: 0 })
    .insert_bundle(SpriteSheetBundle {
      transform: Transform {
        translation: Vec3::new(0.,20.,2.),
        ..default()
      },
      texture_atlas: texture_atlas_handle,
      ..default()
    })
    .with_children(|parent| {
      parent.spawn_bundle(Camera2dBundle {
        transform: Transform {
          scale: Vec3::new(scale.tile_scale, scale.tile_scale, 1.),
          ..default()
        },
        projection:  OrthographicProjection {
          far: 10000.,
          ..default()
        },
        ..default()
      });
    });
}

fn move_player(
  mut events: EventReader<TileClickEvent>,
  mut query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Player), With<Player>>,
  scale: Res<GameScale>
) {
  let r = query.single_mut();
  let mut transform = r.0;
  let mut sprites = r.1;
  let mut player = r.2;


  for e in events.iter() {
    let dx = e.x - player.x;
    let dy = e.y - player.y;

    if        dx != 0 && dy > 0 {
      sprites.index = 4;
    } else if dx != 0 && dy < 0 {
      sprites.index = 1;
    } else if dx == 0 && dy > 0 {
      sprites.index = 3;
    } else if dx == 0 && dy < 0 {
      sprites.index = 2;
    } else {
      sprites.index = 0;
    }

    sprites.flip_x = dx < 0;

    transform.translation.x = e.x as f32 * (scale.tile_size);
    transform.translation.y = e.y as f32 * (scale.tile_size) + 20.;
    player.x = e.x;
    player.y = e.y;
  }
}