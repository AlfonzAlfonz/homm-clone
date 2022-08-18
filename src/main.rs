mod battlefield;
mod world;

use battlefield::*;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::utils::Duration;
use bevy::winit::WinitSettings;
use battlefield::*;
use bevy::prelude::*;
use world::WorldPlugin;

fn main() {
  App::new()
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldPlugin)
    // .add_plugin(BattlefieldPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
  // mut commands: Commands
  mut windows: ResMut<Windows>,
) {
  if let Some(w) = windows.get_primary_mut() {
    w.set_scale_factor_override(Some(2. * w.scale_factor()));
    w.set_resolution(16. * 40., 9. * 40.);
  }
}
