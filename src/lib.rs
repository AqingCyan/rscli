extern crate core;

pub use opts::{Opts, Subcommand};
pub use process::process_csv;
mod opts;
mod process;
