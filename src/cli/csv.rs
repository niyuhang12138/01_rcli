//! csv command
use std::{fmt::Display, str::FromStr};

use clap::Parser;

use crate::{process_csv, verify_file, CmdExecutor};

/// support types of output format
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

/// CSV command
#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_file)]
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

impl CmdExecutor for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // 如果这个output这个字段没有被设置, 则使用output.{format}来作为缺省值
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };

        process_csv(&self.input, output, self.format).await?;
        Ok(())
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
