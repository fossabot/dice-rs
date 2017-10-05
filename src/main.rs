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
  /// A test value
  #[structopt(short = "t", long = "test", default_value = "42")]
  test: f64,
}

use structopt::StructOpt;

fn main() {
  println!("Hi, I'm a work in progress\n");
  let opt = Opt::from_args();
  println!("Here are the configs I got: {:?}", opt);
}
