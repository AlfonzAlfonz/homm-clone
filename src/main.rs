mod game;
mod player;
mod map;

use bevy::render::texture::ImageSettings;
use game::Game;
use bevy::prelude::*;

fn main() {
  App::new()
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_plugin(Game)
    .run();
}

