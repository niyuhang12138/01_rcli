use std::path::PathBuf;

use clap::Parser;

use crate::{process_http_serve, CmdExecutor};

use super::verify_path;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(short, long,value_parser = verify_path)]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
