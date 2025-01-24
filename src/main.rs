use anyhow::Result;
use clap::Parser;
use crb::agent::Runnable;
use hug::{HubApp, RunArgs};

#[tokio::main]
async fn main() -> Result<()> {
    let args = RunArgs::parse();
    HubApp::new(args).run().await;
    // Unblocking stdin
    std::process::exit(0);
}
