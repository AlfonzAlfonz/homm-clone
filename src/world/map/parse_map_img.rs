use bevy::prelude::*;

pub fn parse_map_img(img: &Image) -> MapData {
  let width = img.texture_descriptor.size.width;

  let mut tiles = Vec::new();

  for i in 0..width * width {
    let offset = i as usize * 4;
    let color = (img.data[offset], img.data[offset + 1], img.data[offset + 2]);

    match color {
      (34, 52, 23) => tiles.push(MapTile::Grass),
      (251, 255, 147) => tiles.push(MapTile::Sand),
      (34, 52, 255) => tiles.push(MapTile::Water),
      (255, 255, 255) => tiles.push(MapTile::Snow),
      (61, 28, 6) => tiles.push(MapTile::Dirt),
      (33, 33, 33) => tiles.push(MapTile::Necro),
      (r, g, b) => tiles.push(MapTile::Color(r, g, b)),
    }
  }

  MapData {
    tiles,
    width: width as usize,
  }
}

pub struct MapData {
  tiles: Vec<MapTile>,
  width: usize,
}

impl MapData {
  pub fn at(&self, x: usize, y: usize) -> &MapTile {
    if self.width * y + x >= self.width * self.width {
      return &MapTile::Color(255, 0, 255);
    }
    &self.tiles[self.width * y + x]
  }
}

#[derive(Clone)]
pub enum MapTile {
  Grass,
  Sand,
  Water,
  Snow,
  Dirt,
  Necro,

  Color(u8, u8, u8),
}
