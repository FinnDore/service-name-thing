use std::time::Duration;

use tokio::time::sleep;
use tracing::{info, instrument};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let axiom_layer = tracing_axiom::builder().with_service_name("fmt").layer()?;
    let fmt_layer = tracing_subscriber::fmt::layer().pretty();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(axiom_layer)
        .try_init()?;

    loop {
        say_hello().await;
    }
}

#[instrument]
async fn say_hello() {
    info!("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    sleep(Duration::from_secs(1)).await;
}
