use std::env;

use ssal::interface::{rollup, sequencer};
use ssal_core::{
    axum::{
        routing::{get, post},
        Router, Server,
    },
    error::{Error, WrapError},
    tokio,
    tower_http::cors::{Any, CorsLayer},
    tracing_subscriber,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("sequencer/register", post(sequencer::register))
        .route("rollup/register", post(rollup::register))
        .route("rollup/close_block", post(rollup::close_block))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    Server::bind(([0, 0, 0, 0], 0))
        .serve(app.into_make_service())
        .await
        .wrap("Failed to start the SSAL server");

    // Server::bind().await.wrap?;
    Ok(())
}
