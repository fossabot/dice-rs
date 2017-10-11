extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;

pub(crate) fn gen(max: u32) -> u32 {
  use rand::{thread_rng, Rng};
  thread_rng().gen_range(0, max)
}

pub mod system_gen;
pub mod config;

use config::{Command, Config, Opt};
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;

// TODO allow use of '?' in main
fn main() {
  println!("Hi, I'm a work in progress\n");
  let opt = Opt::from_args();
  println!("I will try to get a config file from: '{}'", opt.config);
  if let Some(cmd) = opt.cmd {
    match cmd {
      Command::WriteDefault => {
        println!("Writting default config to '{}'", opt.config);
        let mut file = File::create(opt.config).expect("unable to open config file");
        file
          .write_all(
            toml::to_string_pretty(&Config::default())
              .expect("unable to make default config")
              .as_bytes(),
          )
          .expect("unable to write defaults to config file");
      }
    }
  }
}
