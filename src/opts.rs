use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    CSV(CsvOpts),
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(long, value_parser = verify_file_input)]
    pub input: String,
    #[arg(long, default_value = "output.json")]
    pub output: String,
    #[arg(long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_input(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}
