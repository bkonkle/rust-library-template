use anyhow::Result;
use std::sync::Arc;

use crate::{
    utils::{config::Config},
};

/// Dependencies needed by the resolvers
pub struct Context {
    /// The app config
    pub config: &'static Config,
}

/// Intialize dependencies
impl Context {
    /// Create a new set of dependencies based on the given shared resources
    pub async fn init(config: &'static Config) -> Result<Self> {
        Ok(Self {
            config,
        })
    }
}
