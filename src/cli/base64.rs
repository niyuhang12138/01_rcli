//! base64 encode/decode commands
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode, verify_file, CmdExecutor};

/// base64 encode/decode commands
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts),
}

/// base64 encode command
#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_encode(&self.input, self.format)?;
        println!("{encode}");
        Ok(())
    }
}

/// base64 decode command
#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_decode(&self.input, self.format)?;
        let decode = String::from_utf8(decode)?;
        println!("{decode}");
        Ok(())
    }
}

/// command receiving format
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
