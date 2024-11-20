use anyhow::Result;
use relayer::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = RelayerConfig::new().await;
    run(config).await?;

    Ok(())
}
