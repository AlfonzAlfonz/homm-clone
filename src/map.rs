use crate::player::Player;
use bevy::input::mouse::MouseButtonInput;
use crate::game::GameScale;
use bevy::prelude::*;
use rand::thread_rng;
use rand::Rng;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_event::<TileClickEvent>()
      .add_system(on_tile_click);
  }
}

#[derive(Component)]
pub struct Map;

fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
  scale: Res<GameScale>
) {
  let texture_handle = asset_server.load("ProjectUtumno_full.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 64, 95);

  let mut e = commands.spawn();
  
  e.insert(Map);

  let mut rng = thread_rng();
  
  for x in 0..32 {
    for y in 0..32 {
      commands.spawn()
        .insert_bundle(SpriteSheetBundle {
          transform: Transform {
            translation: Vec3::new(x as f32 * scale.tile_size, y as f32 * scale.tile_size, 1.),
            scale: Vec3::new(1.,1.,1.),
            ..default()
          },
          sprite: TextureAtlasSprite::new(64 * 9 + 15 + rng.gen_range(0..20)),
          texture_atlas: texture_atlases.add(texture_atlas.clone()),
          ..default()
        });
    }
  }
}

fn on_tile_click(
  mut mouse_button_input_events: EventReader<MouseButtonInput>,
  mut tile_click_events: EventWriter<TileClickEvent>,
  windows: Res<Windows>,
  transform: Query<&Transform, With<Player>>,
  scale: Res<GameScale>
) {
  if let Some(window) = windows.get_primary() {
    if let Some(pos) = window.cursor_position() {
      let w = window.width();
      let h = window.height();
      for e in mouse_button_input_events.iter() {
        if e.state.is_pressed() {
          let abs = resolve_abs_pos(pos, transform.single().translation, w, h, &scale);
          tile_click_events.send(TileClickEvent{
            x: (abs.x / (scale.tile_size / scale.tile_scale)).floor() as i32, 
            y: (abs.y / (scale.tile_size / scale.tile_scale)).floor() as i32
          });
        }
      }
    }
  }

  fn resolve_abs_pos(
    cursor_position: Vec2,
    translation: Vec3,
    win_w: f32,
    win_h: f32,
    scale: &Res<GameScale>
  ) -> Vec2 {
    Vec2::new(
      (cursor_position.x - (win_w as f32 / 2.)) + (scale.tile_size / scale.tile_scale / 2.) + (translation.x / scale.tile_scale), 
      (cursor_position.y - (win_h as f32 / 2.)) + (scale.tile_size / scale.tile_scale / 2.) + (translation.y / scale.tile_scale),
    )
  }
}

pub struct TileClickEvent {
  pub x: i32,
  pub y: i32
}