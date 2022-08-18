mod battlefield;
mod game;
mod map;
mod player;

use battlefield::*;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use game::Game;

fn main() {
  App::new()
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_plugin(BattlefieldPlugin)
    // .add_plugin(Game)
    .run();
}
