//rcli jwt sign -- sub acme -- aud device1 -- exp 14d
//rcli jwt verify -t <token-value>
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{process_jwt_sign, process_jwt_verify, CmdExector};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JsonWebTokenSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(JsonWebTokenSignOpts),
    #[command(name = "verify", about = "Verify a message with a public/shared key")]
    Verify(JsonWebTokenVerifyOpts),
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JsonWebTokenSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JsonWebTokenVerifyOpts {
    #[arg(long)]
    pub token: String,
}

impl CmdExector for JsonWebTokenSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(self.sub, self.aud, self.exp)?;
        println!("token: {}", token);
        Ok(())
    }
}

impl CmdExector for JsonWebTokenVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_verify(self.token)?;
        Ok(())
    }
}