use anyhow::Result;
use clap::Parser;
use std::{fmt::Display, path::Path, str::FromStr};
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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    /// Output file path
    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>,
    /// Format of output type
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// CSV has header or not
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File docs not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
