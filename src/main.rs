extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;

pub(crate) fn dice(size: u32) -> u32 {
  use rand::{thread_rng, Rng};
  thread_rng().gen_range(0, size)
}

pub(crate) fn gen(min: u32, max: u32) -> u32 {
  use rand::{thread_rng, Rng};
  thread_rng().gen_range(min, max)
}

pub mod system_gen;
pub mod config;
pub mod error;
pub use self::error::{Error, Result};

use config::{Command, Config, Opt};
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;

fn run(opt: Opt) -> Result<()> {
  println!("I will try to get a config file from: '{}'", opt.config);
  match opt.cmd {
    Some(Command::WriteDefault) => {
      println!("Writting default config to '{}'", opt.config);
      let mut file = File::create(opt.config)?;
      file.write_all(toml::to_string_pretty(&Config::default())?.as_bytes())?;
    }
    None => {}
  }
  Ok(())
}

fn main() {
  println!("Hi, I'm a work in progress");
  ::std::process::exit(match run(Opt::from_args()) {
    Ok(_) => 0,
    Err(e) => {
      eprintln!("Prgram exited with error: {}", e);
      1
    }
  });
}
