use clap::Parser;
use std::{fmt, str::FromStr};

use super::verify_file_input;

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(long, value_parser = verify_file_input)]
    pub input: String,
    #[arg(long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            &_ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormat::JSON),
    //     "yaml" => Ok(OutputFormat::YAML),
    //     &_  => Err(anyhow::anyhow!("Invalid format")),
    // }
    format.parse()
}
