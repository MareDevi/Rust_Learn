use clap::Parser;
use std::fmt;
use std::str::FromStr;

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum Outputformat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: Outputformat,
    
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn parse_format(format: &str) -> Result<Outputformat, anyhow::Error> {
    format.parse::<Outputformat>()
}

impl From<Outputformat> for &'static str {
    fn from(format: Outputformat) -> Self {
        match format {
            Outputformat::Json => "json",
            Outputformat::Yaml => "yaml",
            Outputformat::Toml => "toml",
        }
    }
}

impl FromStr for Outputformat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Outputformat::Json),
            "yaml" => Ok(Outputformat::Yaml),
            "toml" => Ok(Outputformat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for Outputformat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
} 