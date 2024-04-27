use std::{fs::File, io::Read};

use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    prelude::*,
};

use crate::cli::base64::Base64Format; // import the base64::prelude module to use the encode and decode functions without specifying the full path.
pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?; // get a reader for the input.
    println!("reader - 3");
    let mut buf = Vec::new(); // create a buffer to hold the file contents.
    reader.read_to_end(&mut buf)?; // read the file contents into the buffer.
    println!("buf - 4");
    let encoded = match format {
        // encode the buffer contents according to the specified format.
        Base64Format::StanDand => STANDARD.encode(&buf), // use the standard base64 encoding.
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf), // use the URL-safe base64 encoding.
    };

    println!("Encoded: {}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?; // get a reader for the input.
    let mut buf = Vec::new(); // create a buffer to hold the file contents.
    reader.read_to_end(&mut buf)?; // read the file contents into the buffer.
    let decoded = match format {
        // encode the buffer contents according to the specified format.
        Base64Format::StanDand => STANDARD.decode(&buf)?, // use the standard base64 encoding.
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?, // use the URL-safe base64 encoding.
    };

    let decoded = String::from_utf8(decoded)?;

    println!("Decoded: {}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // create a function to get a reader for the input.
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?) // wrap the file in a Box to make it suitable for the reader parameter.
    };
    Ok(reader)
}
