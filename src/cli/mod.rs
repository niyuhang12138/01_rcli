mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about = "This is a cli with a lot of commands", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
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
