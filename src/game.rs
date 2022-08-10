// use bevy::sprite::MaterialMesh2dBundle;
use crate::map::MapPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;

pub struct Game;

pub struct GameScale {
  pub tile_size: f32,
  pub tile_scale: f32
}

impl Plugin for Game {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameScale { tile_size: 32., tile_scale: 0.75 })
      .add_plugin(MapPlugin)
      .add_plugin(PlayerPlugin)
      .add_startup_system(setup);
  }
}

fn setup(
  // mut commands: Commands
) {
}