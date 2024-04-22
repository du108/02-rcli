mod base64;
mod opts;
mod process;

pub use opts::{Args, OutputFormat, SubCommand};
pub use process::{process_csv, process_genpass};
