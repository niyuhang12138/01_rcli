mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use crate::CmdExecutor;
use std::path::{Path, PathBuf};

// pub use self::base64::{Base64DecodeOpts, Base64EncodeOpts};
use self::csv::CsvOpts;
pub use self::csv::OutputFormat;
pub use base64::{Base64Format, Base64SubCommand};
use clap::Parser;
use genpass::GenPassOpts;
pub use http::HttpSubCommand;
pub use text::{TextSignFormat, TextSubCommand};

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Http server")]
    Http(HttpSubCommand),
}

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await?,
            SubCommand::GenPass(opts) => opts.execute().await?,
            SubCommand::Base64(cmd) => cmd.execute().await?,
            SubCommand::Text(cmd) => cmd.execute().await?,
            SubCommand::Http(cmd) => cmd.execute().await?,
        }
        Ok(())
    }
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File docs not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        println!("path: {path}");
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File docs not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File docs not exist"));
    }
}
