//! http command
use crate::{process_http_serve, verify_path, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

/// http command
#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

/// serve command
#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(short, long,value_parser = verify_path)]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}
