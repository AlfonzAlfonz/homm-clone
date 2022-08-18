use bevy::prelude::*;

pub struct BgPlugin;

impl Plugin for BgPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup);
  }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn()
    .insert_bundle(TransformBundle {
      local: Transform {
        translation: Vec3::new(0., 0., -1.),
        scale: Vec3::new(0.05, 0.05, 1.),
        ..default()
      },
      ..default()
    })
    .insert_bundle(VisibilityBundle { ..default() })
    .with_children(|parent| {
      for x in 0..5 {
        for y in 0..5 {
          parent.spawn_bundle(SpriteBundle {
            transform: Transform {
              translation: Vec3::new(x as f32 * 2048. - 2000., y as f32 * 1365. - 1200., 1.),
              // scale: Vec3::new(1., 1., 1.),
              ..default()
            },
            texture: asset_server.load("battlefield.png"),
            ..default()
          });
        }
      }
    });
}
