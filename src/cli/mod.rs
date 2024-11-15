mod csv;
mod genpass;
mod base64;

use csv::CsvOpts;
use genpass::GenpassOpts;
use clap::Parser;
use std::path::Path;


pub use self::{csv::Outputformat,
    base64::{Base64SubCommand, Base64Format}
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}


#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        return Ok(filename.into());
    } else {
        return Err("File does not exist!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist!"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not_exist.csv"), Err("File does not exist!"));
    }
}