//! # {{project-description}}
#![forbid(unsafe_code)]

mod context;
mod utils;

use anyhow::Result;
use std::sync::Arc;

use context::Context;
use utils::config::get_config;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    // Set RUST_LOG=info (or your desired loglevel) to see logging
    pretty_env_logger::init();

    let config = get_config();
    let context = Arc::new(Context::init(config).await?);

    // TODO: Main routine goes here

    Ok(())
}
