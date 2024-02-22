use std::env;

use ssal::interface::{client::*, rollup::*, sequencer::*};
use ssal_core::{
    axum::{self, routing::post, Router},
    error::{Error, WrapError},
    tokio::{self, net::TcpListener},
    tower_http::cors::CorsLayer,
    tracing, tracing_subscriber,
};
use ssal_database::Database;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let env_variables: Vec<String> = env::args().skip(1).collect();

    // Initialize the database.
    let database_path = env::current_dir()
        .wrap("Failed to get the current directory")?
        .join("database");
    let database = Database::new(database_path)?;

    // Initialize the listener socket.
    let address = env_variables
        .get(0)
        .wrap("Provide the address for the server to bind to")?;
    let listener = TcpListener::bind(address)
        .await
        .wrap(format!("Failed to bind to {:?}", address))?;

    // Set handlers.
    let app = Router::new()
        .route("/client/get-sequencer-set", post(GetSequencerSet::handler))
        .route("/sequencer/register", post(RegisterSequencer::handler))
        .route("/sequencer/get-leader", post(GetLeader::handler))
        .route("/rollup/register", post(RegisterRollup::handler))
        .route("/rollup/deregister", post(DeregisterRollup::handler))
        .route("/rollup/close-block", post(CloseBlock::handler))
        .layer(CorsLayer::permissive())
        .with_state(database);

    // Start the server.
    tracing::info!("Starting the server at {:?}", address);
    axum::serve(listener, app)
        .await
        .wrap("Failed to start the axum server")?;
    Ok(())
}
