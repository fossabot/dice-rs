//!
//! All configurable details about the generator
//!
#[macro_use]
mod macros;

mod cmd;
pub use self::cmd::{Command, Opt};

mod gas;
pub use self::gas::Gas;

mod rocky;
pub use self::rocky::Rocky;

mod star_type;
pub use self::star_type::StarType;

mod config;
pub use self::config::Config;

mod name_list;
pub use self::name_list::DEFAULT_NAME_LIST;

pub use std::collections::BTreeMap as Map;

pub type StarTypes = Map<String, StarType>;
