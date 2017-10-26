//!
//! Generate a star system
//!
pub struct System {
  name: String,
  stars: Vec<Star>,
  objects: Vec<Object>,
}

pub struct Star {
  name: String,
  class: String,
  desc: String,
}

pub struct Object {
  name: String,
  ty: Type,
  in_orbit: Vec<Object>,
}

pub enum Type {
  Planet,
  Belt,
  Gas,
  Cloud,
}

use super::{dice, gen, Config};

pub fn generate(mut c: Config) -> System {
  let num_planets = if (c.chance_avg * 100.0) > dice(100) as f32 {
    c.avg_planets
  } else {
    gen(c.min_planets, c.max_planets)
  };

  let planets: Vec<Object> = Vec::with_capacity(num_planets as usize);

  c.star_types.sort_by_key(|t| (t.chance * 100_000.0) as u32);

  println!("Star Types: {:?}", c.star_types);

  unimplemented!()
}
