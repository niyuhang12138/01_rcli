//! Text signature commands
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
    str::FromStr,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use colored::Colorize;
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::{
    get_content, get_reader, process_text_key_generate, process_text_sign, process_text_verify,
    verify_file, verify_path, CmdExecutor,
};

/// Text signature commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate new key")]
    Generate(TextKeyGenerateOpts),
}

/// Text signature format
#[derive(Parser, Debug)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;
        // base64 output
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("\n{}{}", "signer: ".bright_purple(), encoded.bright_green());
        Ok(())
    }
}

/// Text signature verify
#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("\n{}", "✓ Signature verified".bright_green());
        } else {
            println!("\n{}", "⚠ Signature not verified".bright_red());
        }
        Ok(())
    }
}

/// Text signature key generate
#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output.join(k), v).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Black3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Black3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Black3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
