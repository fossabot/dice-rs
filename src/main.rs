extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;

pub mod system_gen;

pub(crate) fn gen(max: u32) -> u32 {
  use rand::{thread_rng, Rng};
  thread_rng().gen_range(0, max)
}

pub mod config {
  pub use std::collections::BTreeMap as Map;

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Config {
    /// The average amount of planets in a system
    pub avg_planets: u32,

    /// The minimum amount of planets in a system. min < avg
    pub min_planets: u32,

    /// The maximum amount of planets in a system. avg < max
    pub max_planets: u32,

    ///  A list of all possible star types associated to their name
    // TODO see if this is better than a Vec of StarType (name: String)
    pub star_types: Map<String, StarType>,

    /// Options for generating rocky planets
    pub rocky: Rocky,

    /// Options for generating gas giants
    pub gas: Gas,
  }

  macro_rules! map {
    ($($id:expr => $val:expr),+) => ({
      let mut m = Map::new();
      $(
        m.insert(From::from($id), $val);
      )+
      m
    });
  }

  impl Default for Config {
    fn default() -> Self {
      Config {
        avg_planets: 8,
        min_planets: 2,
        max_planets: 12,
        rocky: Rocky::default(),
        gas: Gas::default(),
        star_types: map!(
          // TODO Add more star types
          "G" => StarType {
            part_multy: 0.4,
            make_multy: 0.1,
            rocky: 0.6,
            gas: 0.6,
            belts: 0.2,
          }
        ),
      }
    }
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct StarType {
    /// Chance for the star to be part of a multy star system
    pub part_multy: f32,

    /// Chance for the star to be the cause of a multy star system
    pub make_multy: f32,

    /// Chance for the star to have rocky planets
    pub rocky: f32,

    /// Chance for the star to have gas gigants
    pub gas: f32,

    /// Chance for the star to have asteroid belts
    pub belts: f32,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Rocky {
    /// The chance of a rocky planet having a moon
    pub moon: f32,

    /// The chance of a rocky planet having a belt
    pub belt: f32,
  }

  impl Default for Rocky {
    fn default() -> Self {
      Rocky {
        belt: 0.2,
        moon: 0.4,
      }
    }
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Gas {
    /// The chance of a Gas Giant having a belt
    pub belt: f32,

    /// The average amount of moons a Gas Gigant has.
    pub avg_moon: u32,

    /// The minimum amount of moons a Gas Gigant has. min < avg
    pub min_moon: u32,

    /// The maximum amount of moons a Gas Gigant has. avg < max
    pub max_moon: u32,
  }

  impl Default for Gas {
    fn default() -> Self {
      Gas {
        belt: 0.7,
        avg_moon: 4,
        min_moon: 1,
        max_moon: 12,
      }
    }
  }
}

#[derive(StructOpt)]
#[structopt]
pub struct Opt {
  /// The name of the config file to use for this run
  #[structopt(short = "c", long = "config", default_value = "config.toml")]
  pub config: String,

  #[structopt(subcommand)]
  pub cmd: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
  /// Output the default config to the config file. WARNING! This will override any information in the file
  #[structopt(name = "write-default")]
  WriteDefault,
}

use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  println!("Hi, I'm a work in progress\n");
  let opt = Opt::from_args();
  println!("I will try to get a config file from: '{}'", opt.config);
  if let Some(cmd) = opt.cmd {
    match cmd {
      Command::WriteDefault => {
        println!("Writting default config to '{}'", opt.config);
        let mut file = File::create(opt.config).expect("unable to open config file");
        file.write_all(toml::to_string_pretty(&config::Config::default()).expect("unable to make default config").as_bytes()).expect("unable to write defaults to config file");
      }
    }
  }
}
