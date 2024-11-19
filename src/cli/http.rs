use std::path::PathBuf;
use clap::Parser;
use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command{ about = "Start the HTTP server" }]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg( long, value_parser = verify_path, default_value = ".")]
    pub path: PathBuf,
    #[arg( long, default_value = "8080")]
    pub port: u16,
}