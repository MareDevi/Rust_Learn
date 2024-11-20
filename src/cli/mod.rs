mod csv;
mod genpass;
mod base64;
mod text;
mod http;
use enum_dispatch::enum_dispatch;
use clap::Parser;
use std::path::{Path, PathBuf};
pub use self::{csv::*,
    base64::*,
    text::*,
    http::*,
    genpass::*
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}


#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand, name = "base64", about = "Base64 encode/decode text")]
    Base64(Base64SubCommand),
    #[command(subcommand, name = "text", about = "Text manipulation utilities")]
    Text(TextSubCommand),
    #[command(subcommand, name = "http", about = "HTTP client utilities")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        return Ok(filename.into());
    } else {
        return Err("File does not exist!");
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist!"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not_exist.csv"), Err("File does not exist!"));
    }
}