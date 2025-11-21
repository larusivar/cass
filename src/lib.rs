pub mod config;
pub mod connectors;
pub mod indexer;
pub mod model;
pub mod search;
pub mod storage;
pub mod ui;

use anyhow::Result;

/// Library entrypoint; wire CLI/TUI startup here.
pub async fn run() -> Result<()> {
    Ok(())
}
