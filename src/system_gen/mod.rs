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
use super::config::StarType;

pub fn generate(mut c: Config) -> System {
  c.star_types.sort_by_key(|t| (t.chance * 100_000.0) as u32);

  let star = dice(100_000);
  let mut prev_star: Option<&StarType> = None;
  let mut found_star: Option<&StarType> = None;
  for curr in &c.star_types {
    if let Some(prev) = prev_star {
      if star < (prev.chance * 100_000.0) as u32 && star > (curr.chance * 100_000.0) as u32 {
        found_star = Some(curr);
        break;
      }
    } else if star >= (curr.chance * 100_000.0) as u32 {
      found_star = Some(curr);
      break;
    }

    prev_star = Some(curr);
  }

  if let None = found_star {
    found_star = c.star_types.last();
  }

  if let Some(star) = found_star {
    let mut star_count = 1;
    let multy = dice(100);
    let mut companion: Option<&StarType> = None;
    if (star.make_multy * 100.0) as u32 > multy {
      for curr in &c.star_types {
        if let Some(prev) = prev_star {
          if multy < (prev.part_multy * 100.0) as u32 && multy > (curr.part_multy * 100.0) as u32 {
            companion = Some(curr);
            star_count += 1;
            break;
          }
        } else if multy >= (curr.part_multy * 100.0) as u32 {
          companion = Some(curr);
          star_count += 1;
          break;
        }

        prev_star = Some(curr);
      }
    }

    let mut stars = Vec::with_capacity(star_count);
    stars.push(Star {
      name: "Main".into(),
      class: star.class.clone(),
      desc: star.description.clone(),
    });

    if let Some(comp) = companion {
      stars.push(Star {
        name: "Other".into(),
        class: comp.class.clone(),
        desc: comp.description.clone(),
      });
    }
    
    let num_planets = if (c.chance_avg * 100.0) > dice(100) as f32 {
      c.avg_planets
    } else {
      gen(c.min_planets, c.max_planets)
    };

    let num_objects = num_planets + 2;

    let objects: Vec<Object> = Vec::with_capacity(num_objects as usize);
  }

  unimplemented!()
}
