//! genpass command
use crate::{process_genpass, CmdExecutor};
use clap::Parser;
use colored::Colorize;
use zxcvbn::zxcvbn;

/// genpass command
#[derive(Parser, Debug)]
pub struct GenPassOpts {
    /// generate password length
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// generate password whether support or not uppercase
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    /// generate password whether support or not lowercase
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    /// generate password whether support or not number
    #[arg(long, default_value_t = true)]
    pub number: bool,

    /// generate password whether support or not symbol
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;

        println!("generate password: {}", password.purple());

        // test password strength
        let result = zxcvbn(&password, &[]);
        println!("Password strength: {}", result.score().to_string().red());

        Ok(())
    }
}
