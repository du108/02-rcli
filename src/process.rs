use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
struct Record {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let result = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Record>>();
    let json = serde_json::to_string_pretty(&result)?;
    fs::write(output, json)?;
    Ok(())
}
