extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

pub mod system_gen;

pub(crate) fn gen(max: u32) -> u32 {
  use rand::{thread_rng, Rng};
  thread_rng().gen_range(0, max)
}

#[derive(StructOpt, Debug)]
#[structopt]
struct Opt {
  /// The average amount of planets in a system
  #[structopt(long = "avg", default_value = "8")]
  avg_planets: u32,

  /// The minimum amount of planets in a system. min < avg
  #[structopt(long = "min", default_value = "2")]
  min_planets: u32,

  /// The maximum amount of planets in a system. avg < max
  #[structopt(long = "max", default_value = "12")]
  max_planets: u32,

  // Insert all the stars here, we'll config their chance to be part of multi star, and to make multi star
  // also, per star chance of it having rocky, versus gas, versus belts.

  /// The chance of a rocky planet having a moon
  #[structopt(long = "r-moon", default_value = "0.4")]
  rocky_moon: f32,

  /// The chance of a rocky planet having a belt
  #[structopt(long = "r-belt", default_value = "0.2")]
  rocky_belt: f32,
  
  /// The chance of a Gas Giant having a belt
  #[structopt(long = "g-belt", default_value = "0.7")]
  gas_belt: f32,
  
  /// The average amount of moons a Gas Gigant has.
  #[structopt(long = "g-moon-avg", default_value = "4")]
  avg_gas_moon: u32,
  
  /// The minimum amount of moons a Gas Gigant has. min < avg
  #[structopt(long = "g-moon-min", default_value = "4")]
  min_gas_moon: u32,
  
  /// The maximum amount of moons a Gas Gigant has. avg < max
  #[structopt(long = "g-moon-max", default_value = "4")]
  max_gas_moon: u32,
}

use structopt::StructOpt;

fn main() {
  println!("Hi, I'm a work in progress\n");
  let opt = Opt::from_args();
  println!("Here are the configs I got: {:?}", opt);
}
