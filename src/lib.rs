mod cli;
mod process;

pub use crate::cli::{base64::Base64SubCommand, Args, SubCommand}; // re-exporting the structs and enums from the cli module
pub use process::{process_csv, process_decode, process_encode, process_genpass};
