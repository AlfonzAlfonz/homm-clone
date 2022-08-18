mod parse_map_img;
mod tiles;

use bevy::input::mouse::MouseButtonInput;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{asset::LoadState, prelude::*, render::render_resource::*};
use bevy_mod_raycast::RayCastMesh;
use bevy_mod_raycast::*;
use parse_map_img::*;
use tiles::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_event::<MapClickEvent>()
      .add_plugin(DefaultRaycastingPlugin::<MapRaycastSet>::default())
      .add_system_to_stage(
        CoreStage::First,
        update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MapRaycastSet>),
      )
      .add_system(on_tile_click)
      .add_system(create_map);
  }
}

#[derive(Component)]
pub struct Map;

pub struct MapRaycastSet;

#[derive(Debug)]
pub struct MapClickEvent {
  pub x: isize,
  pub y: isize,
}

pub struct MapResource {
  loaded: bool,
  bg_handle: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.insert_resource(MapResource {
    loaded: false,
    bg_handle: asset_server.load("map.png"),
    // bg_handle: asset_server.load("solid-color-image.png"),
  });

  commands.spawn().insert(Map);
}

pub const TILE_SIZE: usize = 24;

fn create_map(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut map_res: ResMut<MapResource>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  if map_res.loaded || asset_server.get_load_state(map_res.bg_handle.clone()) != LoadState::Loaded {
    return;
  }
  map_res.loaded = true;

  info!("start");

  let map = images.get_mut(&map_res.bg_handle).unwrap();
  info!("{:?}", map.texture_descriptor);
  let map_size = map.texture_descriptor.size.width as usize;
  let width = map_size * TILE_SIZE;

  let data = parse_map_img(map);

  let mut image = Image::new_fill(
    Extent3d {
      width: (map_size * TILE_SIZE) as u32,
      height: (map_size * TILE_SIZE) as u32,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &[0, 0, 0, 255],
    TextureFormat::Rgba8UnormSrgb,
  );
  // image.texture_descriptor.usage =
  //   TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

  let mut noise = MapNoises::new(width, 0);

  image.data = (0..width * width)
    .flat_map(|i| {
      let x = i % width;
      let y = i / width;

      if x % 24 == 0 || y % 24 == 0 {
        return noise.water_tile(x, y);
      }

      let tile = data.at(x / TILE_SIZE, y / TILE_SIZE);


      if x < 18 || y < 18 || x > width - 18 || y > width - 18 {
        return noise.match_tile(x, y, tile);
      }

      let x_m = (x as f32) / 24.;
      let y_m = (y as f32) / 24.;

      let tile_a = data.at(x_m.ceil() as usize, y_m.ceil() as usize);
      let tile_b = data.at(x_m.floor() as usize, y_m.ceil() as usize);
      let tile_c = data.at(x_m.ceil() as usize, y_m.floor() as usize);
      let tile_d = data.at(x_m.floor() as usize, y_m.floor() as usize);

      let tile = noise.mix_texture(x, y, tile_a, tile_b, tile_c, tile_d);

      noise.match_tile(x, y, tile)
    })
    .collect();

  let image_handle = images.add(image);

  commands
    .spawn()
    .insert_bundle(MaterialMesh2dBundle {
      mesh: meshes
        .add(Mesh::from(shape::Quad {
          size: Vec2::new(width as f32, width as f32),
          flip: false,
        }))
        .into(),
      material: materials.add(ColorMaterial {
        texture: Some(image_handle),
        ..default()
      }),
      ..default()
    })
    .insert(Map)
    .insert(RayCastMesh::<MapRaycastSet>::default());

  info!("done");
}
fn on_tile_click(
  mut mouse_button_input_events: EventReader<MouseButtonInput>,
  mut tile_click_events: EventWriter<MapClickEvent>,
  sources: Query<&RayCastSource<MapRaycastSet>>,
) {
  for e in mouse_button_input_events.iter() {
    if e.state.is_pressed() {
      for (_, i) in sources.iter().filter_map(|source| source.intersect_top()) {
        tile_click_events.send(MapClickEvent {
          x: (i.position().x / TILE_SIZE as f32).floor() as isize,
          y: (i.position().y / TILE_SIZE as f32).floor() as isize,
        });
      }
    }
  }
}

fn update_raycast_with_cursor(
  mut cursor: EventReader<CursorMoved>,
  mut query: Query<&mut RayCastSource<MapRaycastSet>>,
) {
  let cursor_position = match cursor.iter().last() {
    Some(cursor_moved) => cursor_moved.position,
    None => return,
  };

  for mut pick_source in &mut query {
    pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
  }
}
