#[derive(StructOpt)]
#[structopt]
pub struct Opt {
  /// The name of the config file to use for this run
  #[structopt(short = "c", long = "config", default_value = "config.toml")]
  pub config: String,

  /// Specify a comma or newline delimited list of names for Star Systems
  #[structopt(short = "n", long = "names")]
  pub name_list: Option<String>,

  #[structopt(subcommand)] pub cmd: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
  /// Output the default config to the config file. WARNING! This will override any information in the file
  #[structopt(name = "write-default")]
  WriteDefault,
}
