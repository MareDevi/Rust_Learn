use serde::{Serialize, Deserialize};
use csv::Reader;
use std::fs;
use anyhow::Result;

use crate::cli::Outputformat;



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String, 
    #[serde(rename = "Position")]
    positon: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8, 
}

pub fn process_csv(input: &str, output: String, format: Outputformat) -> Result<()>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?; 
        let json_value =  headers
        .iter()
        .zip(record.iter())
        .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content = match format {
        Outputformat::Json => serde_json::to_string_pretty(&ret)?,
        Outputformat::Yaml => serde_yaml::to_string(&ret)?,
        Outputformat::Toml => toml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
} 