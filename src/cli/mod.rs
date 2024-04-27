pub mod base64;
mod csv;
mod genpass;

use std::path::Path;

use clap::{Parser, Subcommand};

use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "show CSV or convert csv to other format")]
    CSV(CsvOpts),
    #[command(name = "genpass", about = "Generate a password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "b64")]
    Base64(Base64SubCommand),
}

pub fn verify_file_input(filename: &str) -> Result<String, &'static str> {
    println!("verify filename1");
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
    // if Path::new(filename).exists() {
    //     Ok(filename.into())
    // } else {
    //     Err("File does not exist".into())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_input() {
        assert_eq!(verify_file_input("-"), Ok("-".into()));
        assert_eq!(verify_file_input("cargo.toml"), Ok("cargo.toml".into()));
        assert_eq!(verify_file_input("*"), Err("File does not exist"));
    }
}
