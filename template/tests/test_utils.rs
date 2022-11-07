#![allow(dead_code)] // Since each test is an independent module, this is needed

use anyhow::Result;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::sleep;

use {{crate_name}}::{
    context::Context,
    run,
    utils::{config::get_config, http::http_client},
};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);

/// Creates an http/https client via Hyper
pub fn http_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder().build::<_, Body>(HttpsConnector::new())
}

pub async fn run_server(context: Arc<Context>) -> Result<SocketAddr> {
    let (addr, server) = run(context).await?;

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    Ok(addr)
}

/// Common test utils
pub struct TestUtils {
    pub http_client: &'static Client<HttpsConnector<HttpConnector>>,
    pub ctx: Arc<Context>,
}

impl TestUtils {
    /// Initialize a new set of utils
    pub async fn init() -> Result<Self> {
        let _ = pretty_env_logger::try_init();

        let config = get_config();

        // This needs to be created anew each time because the database connection can't be shared
        // when the Tokio runtime is being stopped and re-started between tests
        let ctx = Arc::new(Context::init(config).await?);

        let http_client = &HTTP_CLIENT;
        let addr = run_server(ctx.clone()).await?;

        Ok(TestUtils {
            http_client,
            ctx,
        })
    }
}
