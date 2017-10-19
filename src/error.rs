pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  IoError(::std::io::Error),
  SerError(::toml::ser::Error),
  DeError(::toml::de::Error),
}

impl From<::std::io::Error> for Error {
  fn from(e: ::std::io::Error) -> Self {
    Error::IoError(e)
  }
}

impl From<::toml::ser::Error> for Error {
  fn from(e: ::toml::ser::Error) -> Self {
    Error::SerError(e)
  }
}

impl From<::toml::de::Error> for Error {
  fn from(e: ::toml::de::Error) -> Self {
    Error::DeError(e)
  }
}

// TODO FIX DISPLAY
impl ::std::fmt::Display for Error {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    writeln!(f, "crash")
  }
}

impl ::std::error::Error for Error {
  fn description(&self) -> &str {
    "crash"
  }
}
