use std::fs;

use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_generate, process_genpass, process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::Genpass(opts) => {
            let password = process_genpass(opts.length,
                            opts.uppercase,
                            opts.lowercase,
                            opts.number,
                            opts.symbol)?;
            println!("{}", password);
        }
        SubCommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    let encoded = process_encode(&opts.input, opts.format)?;
                    println!("{}", encoded);
                }
                Base64SubCommand::Decode(opts) => {
                    let decoded = process_decode(&opts.input, opts.format)?;
                    let decoded = String::from_utf8_lossy(&decoded);
                    println!("{}", decoded);
                }
            }
        }
        SubCommand::Text(subcmd) => {
            match subcmd {
                TextSubCommand::Sign(opts) => {
                    let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                    println!("signed: {}", signed);
                }
                TextSubCommand::Verify(opts) => {
                    let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                    println!("verified: {:?}", verified);
                }
                TextSubCommand::Generate(opts) => {
                    let key = process_generate(opts.format)?;

                    match opts.format {
                        TextSignFormat::Blake3 => {
                            let name = opts.output.join("blake3.txt");
                            fs::write(name, &key[0])?;
                        }
                        TextSignFormat::Ed25519 => {
                            let name = &opts.output;
                            fs::write(name.join("signing_key"), &key[0])?;
                            fs::write(name.join("public_key"), &key[1])?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
