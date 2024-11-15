use std::{fs::File, io::Read};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let encoded = URL_SAFE.encode(&input);
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let input = std::fs::read(input)?;
    let decoded = URL_SAFE.decode(&input)?;
    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}