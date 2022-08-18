use crate::world::map::MapTile;
use noise::*;
use rand::prelude::*;
use rand::rngs::ThreadRng;

pub struct MapNoises {
  size: usize,
  value: Value,
  simplex: OpenSimplex,
  perlin: Perlin,
  rng: ThreadRng,
  worley: RidgedMulti,
}

impl MapNoises {
  pub fn new(size: usize, seed: u32) -> Self {
    let n = MapNoises {
      size,
      value: Value::new(),
      simplex: OpenSimplex::new(),
      perlin: Perlin::new(),
      rng: thread_rng(),
      worley: RidgedMulti::new(),
    };

    n.value.set_seed(seed);
    n.simplex.set_seed(seed);
    n.perlin.set_seed(seed);

    n
  }

  pub fn match_tile(&self, x: usize, y: usize, tile: &MapTile) -> [u8; 4] {
    match tile {
      MapTile::Grass => self.grass_tile(x, y),
      MapTile::Sand => self.sand_tile(x, y),
      MapTile::Necro => self.necro_tile(x, y),
      MapTile::Water => self.water_tile(x, y),
      MapTile::Dirt => self.dirt_tile(x, y),
      MapTile::Snow => self.snow_tile(x, y),
      MapTile::Color(r, g, b) => [*r, *g, *b, 255],
      _ => [255, 0, 255, 255],
    }
  }

  pub fn grass_tile(&self, x: usize, y: usize) -> [u8; 4] {
    // if x % 2 == 0 { [0,0,0,255]} else {[255,255,255,255]}
    let p = [
      self.perlin(x * 3, y * 3) / 60,
      self.perlin(x * 3, y * 3) / 40,
      self.perlin(x * 3, y * 3) / 60,
      0,
    ];

    return self.add(
      [34, 70 + self.value(x * 100000, y * 100000) / 25, 23, 255],
      p,
    );
  }

  pub fn sand_tile(&self, x: usize, y: usize) -> [u8; 4] {
    // if x % 2 == 0 { [0,0,0,255]} else {[255,255,255,255]}
    // let p = [
    //   self.perlin(x * 3, y * 3) / 60,
    //   self.perlin(x * 3, y * 3) / 40,
    //   self.perlin(x * 3, y * 3) / 60,
    //   0,
    // ];

    return [
      (251 - self.value(x * 100000, y * 100000) / 10) as u8,
      (255 - self.value(x * 100000, y * 100000) / 10) as u8,
      (147 - self.value(x * 100000, y * 100000) / 10) as u8,
      255,
    ];
  }

  pub fn necro_tile(&self, x: usize, y: usize) -> [u8; 4] {
    let mut n = (self.worley(x * 100, y * 100) as f32) / 255.;
    // n = (n * 0.5) * (self.perlin(x * 1000, y * 1000) as f32 / 255. * 0.5);
    // let n = (n * 255.) as u8;

    // return [n, n, n, 255];

    let p = self.perlin(x * 1000, y * 1000) / (255 / 55);

    if n > 100. / 255. {
      return [(n * 200.) as u8 + p, 0, 0, 255];
    }

    let p = [
      self.perlin(x * 3, y * 3) / 60,
      self.perlin(x * 3, y * 3) / 40,
      self.perlin(x * 3, y * 3) / 60,
      0,
    ];

    return self.add(
      [
        20 + self.value(x * 100000, y * 100000) / 25,
        10 + self.value(x * 100000, y * 100000) / 25,
        10 + self.value(x * 100000, y * 100000) / 25,
        255,
      ],
      p,
    );
  }

  pub fn water_tile(&self, x: usize, y: usize) -> [u8; 4] {
    // if x % 2 == 0 { [0,0,0,255]} else {[255,255,255,255]}
    let p = [
      self.perlin(x * 3, y * 3) / 60,
      self.perlin(x * 3, y * 3) / 40,
      self.perlin(x * 3, y * 3) / 60,
      0,
    ];

    return [0, 0, 255, 255];
  }

  pub fn dirt_tile(&self, x: usize, y: usize) -> [u8; 4] {
    return [61, 28, 6, 255];
  }

  pub fn snow_tile(&self, x: usize, y: usize) -> [u8; 4] {
    // if x % 2 == 0 { [0,0,0,255]} else {[255,255,255,255]}
    let p = [
      self.perlin(x * 3, y * 3) / 60,
      self.perlin(x * 3, y * 3) / 40,
      self.perlin(x * 3, y * 3) / 60,
      0,
    ];

    return self.sub(
      [
        255 - self.value(x * 100000, y * 100000) / 25,
        255 - self.value(x * 100000, y * 100000) / 25,
        255 - self.value(x * 100000, y * 100000) / 25,
        255,
      ],
      p,
    );
  }

  pub fn perlin(&self, x: usize, y: usize) -> u8 {
    (self
      .perlin
      .get([x as f64 / self.size as f64, y as f64 / self.size as f64])
      * 255.) as u8
  }

  pub fn worley(&self, x: usize, y: usize) -> u8 {
    (self
      .worley
      .get([x as f64 / self.size as f64, y as f64 / self.size as f64])
      * 255.) as u8
  }

  pub fn value(&self, x: usize, y: usize) -> u8 {
    (self
      .value
      .get([x as f64 / self.size as f64, y as f64 / self.size as f64])
      * 255.) as u8
  }

  pub fn simplex(&self, x: usize, y: usize) -> u8 {
    (self
      .simplex
      .get([x as f64 / self.size as f64, y as f64 / self.size as f64])
      * 255.) as u8
  }

  pub fn mult(&self, a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
  }

  pub fn add(&self, a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
  }

  pub fn sub(&self, a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
  }

  // a b
  // c d
  pub fn mix_texture<'a>(
    &mut self,
    x: usize,
    y: usize,
    a: &'a MapTile,
    b: &'a MapTile,
    c: &'a MapTile,
    d: &'a MapTile,
  ) -> &'a MapTile {
    let (x_d, y_d) = (x % 24, y % 24);

    let a_d = ((x_d * x_d + y_d * y_d) as f32).sqrt();
    let b_d = (((24 - x_d) * (24 - x_d) + y_d * y_d) as f32).sqrt();
    let c_d = ((x_d * x_d + (24 - y_d) * (24 - y_d)) as f32).sqrt();
    let d_d = (((24 - x_d) * (24 - x_d) + (24 - y_d) * (24 - y_d)) as f32).sqrt();
    // let a_d = (x_d + y_d) as f32 / 2.;
    // let b_d = (((24 - x_d) + y_d) as f32) / 2.;
    // let c_d = ((x_d + (24 - y_d)) as f32) / 2.;
    // let d_d = (((24 - x_d) + (24 - y_d)) as f32) / 2.;

    let a_n = (self.rng.next_u32() as f32 / (u32::MAX) as f32) * 255.;
    let b_n = (self.rng.next_u32() as f32 / (u32::MAX) as f32) * 255.;
    let c_n = (self.rng.next_u32() as f32 / (u32::MAX) as f32) * 255.;
    let d_n = (self.rng.next_u32() as f32 / (u32::MAX) as f32) * 255.;

    let mut last = 0.;
    let mut result = &MapTile::Grass;

    for (x, r) in [
      (inter(a_d) + a_n, a),
      (inter(b_d) + b_n, b),
      (inter(c_d) + c_n, c),
      (inter(d_d) + d_n, d),
    ] {
      if x > last {
        last = x;
        result = r;
      }
    }

    result
  }
}

const D_MAX: f32 = 24. * 24. + 24. * 24.;

fn inter(x: f32) -> f32 {
  // x * 5.
  (x * 1.4 / D_MAX.sqrt()).powf(1.) * D_MAX.sqrt() * 20.
}

fn inter2(x: f32) -> f32 {
  (x * 1.5 / 255.).powf(1./4.) * 255. * 2.
}

impl Default for MapNoises {
  fn default() -> Self {
    MapNoises {
      size: 0,
      value: Value::new(),
      simplex: OpenSimplex::new(),
      perlin: Perlin::new(),
      rng: thread_rng(),
      worley: RidgedMulti::new(),
    }
  }
}
