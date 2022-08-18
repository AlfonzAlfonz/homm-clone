use crate::world::map::MapRaycastSet;
use bevy_mod_raycast::RayCastSource;
use crate::world::GameScale;
use bevy::prelude::*;

pub struct PlayerCameraPlugin;

const TRESHOLD: f32 = 40.;

impl Plugin for PlayerCameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup).add_system(move_camera);
  }
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup(mut commands: Commands, scale: Res<GameScale>) {
  commands
    .spawn()
    .insert(PlayerCamera)
    .insert_bundle(Camera2dBundle {
      transform: Transform {
        translation: Vec3::new(0., 0., 10.),
        scale: Vec3::new(scale.tile_scale, scale.tile_scale, 1.),
        ..default()
      },
      projection: OrthographicProjection {
        far: 10000.,
        ..default()
      },
      ..default()
    })
    .insert(RayCastSource::<MapRaycastSet>::new());
}

fn move_camera(mut query: Query<&mut Transform, With<PlayerCamera>>, windows: Res<Windows>) {
  let mut transform = query.single_mut();

  if let Some(window) = windows.get_primary() {
    if let Some(pos) = window.cursor_position() {
      let w = window.width();
      let h = window.height();

      if pos.x < TRESHOLD {
        transform.translation.x -= 12.;
      }

      if pos.x > w - TRESHOLD {
        transform.translation.x += 12.;
      }

      if pos.y < TRESHOLD {
        transform.translation.y -= 12.;
      }

      if pos.y > h - TRESHOLD {
        transform.translation.y += 12.;
      }
    }
  }
}
