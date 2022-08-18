mod grid;
mod bg;

use crate::battlefield::grid::*;
use crate::battlefield::bg::*;
use bevy::prelude::*;

pub struct BattlefieldPlugin;

impl Plugin for BattlefieldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(BgPlugin)
      .add_plugin(GridPlugin)
      .add_startup_system(setup)
      .add_system(scale_camera);
  }
}

fn setup(// mut commands: Commands
) {
}

fn scale_camera(
  mut last_width: Local<f32>,
  mut last_height: Local<f32>,
  windows: Res<Windows>,
  mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
  if let Some(w) = windows.get_primary() {
    let (width, height) = (w.width(), w.height());

    if *last_width == width && *last_height == height {
      return;
    }
    let tile_width = 2. * f32::sqrt(3. / 4.);
    let grid_width = tile_width * 10. * (GRID_N_X as f32 - 0.5);
    let grid_height = 11. * 10. * 1.5;
    let scale = if (width / grid_width) < (height / grid_height) {
      1. / (width / grid_width)
    } else {
      1. / (height / grid_height)
    };
    for mut t in camera_query.iter_mut() {
      t.scale.x = scale;
      t.scale.y = scale;
    }
    *last_width = width;
    *last_height = height;
  }
}
