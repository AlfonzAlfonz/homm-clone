use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_raycast::*;

use bevy::render::mesh::*;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridTile {
  x: usize,
  y: usize,
}

pub struct GridRaycastSet;

impl Plugin for GridPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(DefaultRaycastingPlugin::<GridRaycastSet>::default())
      .add_system_to_stage(
        CoreStage::First,
        update_raycast_with_cursor.before(RaycastSystem::BuildRays::<GridRaycastSet>),
      )
      .add_startup_system(setup)
      .add_event::<TileClickEvent>()
      .add_system(on_tile_click);
  }
}

pub const TILE_SCALE: f32 = 7.;
pub const GRID_N_X: usize = 16;
pub const GRID_N_Y: usize = 13;

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let camera = Camera2dBundle { ..default() };

  commands
    .spawn_bundle(camera)
    .insert(RayCastSource::<GridRaycastSet>::new());

  let tile_width: f32 = 2. * f32::sqrt(3. / 4.);

  let total_width = tile_width * TILE_SCALE * (GRID_N_X as f32 - 0.5);
  let total_height = 1.5 * TILE_SCALE * (GRID_N_Y as f32 - 1.);

  for x in 0..GRID_N_X {
    for y in 0..GRID_N_Y {
      let p1 = tile_translate(x as f32, y as f32);

      commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
          mesh: meshes.add(outline_mesh()).into(),
          transform: Transform {
            translation: Vec3::new(
              p1.0 * TILE_SCALE - (total_width / 2.),
              p1.1 * TILE_SCALE - (total_height / 2.) - 10.,
              1.,
            ),
            scale: Vec3::new(TILE_SCALE, TILE_SCALE, 1.),
            ..default()
          },
          material: materials.add(ColorMaterial::from(Color::rgba(1., 1., 1., 1.))),
          ..default()
        })
        // .insert_bundle(PolylineBundle {
        //   transform: Transform {
        //     translation: Vec3::new(
        //       p1.0 * TILE_SCALE - (total_width / 2.),
        //       p1.1 * TILE_SCALE - (total_height / 2.) - 10.,
        //       1.,
        //     ),
        //     scale: Vec3::new(TILE_SCALE, TILE_SCALE, 1.),
        //     ..default()
        //   },
        //   polyline: polylines.add(Polyline {
        //     vertices: vec![
        //       Vec3::new(-extent_x, -extent_y, 0.0]),
        //       Vec3::new(-extent_x, extent_y, 0.0]),
        //       Vec3::new(extent_x, extent_y, 0.0),
        //       Vec3::new(extent_x, -extent_y, 0.0),
        //       Vec3::new(0., extent_y + 0.5, 0.0),
        //       Vec3::new(0., -(extent_y + 0.5), 0.0),
        //     ],
        //     ..Default::default()
        //   }),
        //   material: polyline_materials.add(PolylineMaterial {
        //     width: 3.0,
        //     color: Color::RED,
        //     perspective: true,
        //     ..Default::default()
        //   }),
        //   ..Default::default()
        // })
        .with_children(|parent| {
          parent
            .spawn()
            .insert_bundle(MaterialMesh2dBundle {
              mesh: meshes.add(click_mesh()).into(),
              material: materials.add(ColorMaterial::from(Color::rgba(0., 0., 0., 0.1))),
              ..default()
            })
            .insert(GridTile { x, y })
            .insert(RayCastMesh::<GridRaycastSet>::default());
        });
    }
  }
}

fn tile_translate(x: f32, y: f32) -> (f32, f32) {
  let tile_width: f32 = 2. * f32::sqrt(3. / 4.);

  let offset = if y % 2. == 1. { tile_width / 2. } else { 0. };

  (x * tile_width + offset, 1.5 * y)
}

fn tile_vertices() -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
  let extent_x = f32::sqrt(3. / 4.);
  let extent_y = 0.5;

  let (u_left, u_right) = (0.0, 1.0);
  [
    ([-extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
    ([-extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [u_left, 0.0]),
    ([extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [u_right, 0.0]),
    ([extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [u_right, 1.0]),
    ([0., extent_y + 0.5, 0.0], [0.0, 0.0, 1.0], [u_left, 0.0]),
    ([0., -(extent_y + 0.5), 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
  ]
}

fn outline_mesh() -> Mesh {
  let mut mesh = Mesh::new(PrimitiveTopology::LineList);

  let indices = Indices::U32(vec![1, 4, 4, 2, 2, 3, 3, 5, 5, 0, 0, 1]);

  mesh.set_indices(Some(indices));
  mesh_attributes(mesh)
}

fn click_mesh() -> Mesh {
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

  let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2, 2, 4, 1, 0, 5, 3]);

  mesh.set_indices(Some(indices));
  mesh_attributes(mesh)
}

fn mesh_attributes(mut mesh: Mesh) -> Mesh {
  let vertices = tile_vertices();

  let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
  let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
  let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
  mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
  mesh
}

fn on_tile_click(
  mut mouse_button_input_events: EventReader<MouseButtonInput>,
  mut tile_click_events: EventWriter<TileClickEvent>,
  tile_query: Query<&GridTile>,
  sources: Query<&RayCastSource<GridRaycastSet>>,
) {
  for e in mouse_button_input_events.iter() {
    if e.state.is_pressed() {
      for (e, _) in sources.iter().filter_map(|source| source.intersect_top()) {
        if let Ok(tile) = tile_query.get(e) {
          tile_click_events.send(TileClickEvent {
            x: tile.x,
            y: tile.y,
          });
        }
      }
    }
  }
}

fn update_raycast_with_cursor(
  mut cursor: EventReader<CursorMoved>,
  mut query: Query<&mut RayCastSource<GridRaycastSet>>,
) {
  let cursor_position = match cursor.iter().last() {
    Some(cursor_moved) => cursor_moved.position,
    None => return,
  };

  for mut pick_source in &mut query {
    pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
  }
}

pub struct TileClickEvent {
  pub x: usize,
  pub y: usize,
}
