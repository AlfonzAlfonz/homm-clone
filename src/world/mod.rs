mod map;
mod player;
mod player_camera;

use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_mod_raycast::*;
use map::*;
use player::PlayerPlugin;
use player_camera::PlayerCameraPlugin;

pub struct WorldPlugin;

pub struct GameScale {
  pub tile_size: f32,
  pub tile_scale: f32,
}

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameScale {
        tile_size: 32.,
        tile_scale: 1.,
      })
      .add_plugin(MapPlugin)
      .add_plugin(PlayerPlugin)
      .add_plugin(PlayerCameraPlugin)
      .add_startup_system(setup);
  }
}

fn setup(// mut commands: Commands
) {
}

