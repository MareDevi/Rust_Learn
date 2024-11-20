use core::fmt;
use std::{path::PathBuf, str::FromStr};
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_decrypt, process_encrypt, process_text_sign, process_text_verify, CmdExector};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a message with a public/shared key")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(name = "encrypt", about = "Encrypt a message with a shared key")]
    Encrypt(TextEncryptOpts),
    #[command(name = "decrypt", about = "Decrypt a message with a shared key")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "-")]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "-")]
    pub output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
    
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("verified: {:?}", verified);
        Ok(())
    }
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_generate(self.format)?;       
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                std::fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                std::fs::write(name.join("signing_key"), &key[0])?;
                std::fs::write(name.join("public_key"), &key[1])?;
            }
        }
        Ok(())
    }
}

impl CmdExector for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_encrypt(&self.input, &self.key, &self.output)?;
        Ok(())
    }
}

impl CmdExector for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_decrypt(&self.input, &self.key, &self.output)?;
        Ok(())
    }
}