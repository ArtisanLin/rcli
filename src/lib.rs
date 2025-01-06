pub mod opts;
pub mod process;
pub use opts::{Opts, SubCommand, GenPassOpts};
pub use process::{process_csv, process_gen_pass};
