use anyhow::Result;
use crb::agent::Runnable;
use hug::HubApp;

#[tokio::main]
async fn main() -> Result<()> {
    HubApp::new().run().await;
    // Unblocking stdin
    std::process::exit(0);
}
