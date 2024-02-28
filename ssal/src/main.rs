use std::env;

use ssal::interface::*;
use ssal_core::{
    axum::{
        self,
        routing::{get, post},
        Router,
    },
    error::{Error, WrapError},
    tokio::{self, net::TcpListener},
    tower_http::cors::CorsLayer,
    tracing, tracing_subscriber,
    types::RollupSet,
};
use ssal_database::Database;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let env_variables: Vec<String> = env::args().skip(1).collect();

    // Initialize the listener socket.
    let address = env_variables
        .get(0)
        .wrap("Provide the address for the server to bind to")?;
    let listener = TcpListener::bind(address)
        .await
        .wrap(format!("Failed to bind to {:?}", address))?;

    // Initialize the database.
    let database_path = env::current_dir()
        .wrap("Failed to get the current directory")?
        .join("databases/ssal");
    let database = Database::new(database_path)?;

    // Initialize the rollup set.
    let rollup_set = RollupSet::default();
    database.put(&"rollup_set", &rollup_set)?;

    // Set handlers.
    let app = Router::new()
        .route("/close-block", post(CloseBlock::handler))
        .route("/get-sequencer-set", get(GetSequencerSet::handler))
        .route(
            "/get-closed-sequencer-set",
            get(GetClosedSequencerSet::handler),
        )
        .route("/register-rollup", post(RegisterRollup::handler))
        .route("/register-sequencer", post(RegisterSequencer::handler))
        .layer(CorsLayer::permissive())
        .with_state(database);

    // Start the server.
    tracing::info!("Starting the server at {:?}", address);
    axum::serve(listener, app)
        .await
        .wrap("Failed to start the axum server")?;
    Ok(())
}
