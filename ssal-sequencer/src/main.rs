use std::env;

use ssal_core::{
    axum::{
        self,
        routing::{get, post},
        Router,
    },
    error::{Error, WrapError},
    reqwest::Url,
    tokio::{self, net::TcpListener},
    tower_http::cors::CorsLayer,
    tracing, tracing_subscriber,
    types::*,
};
use ssal_database::Database;
use ssal_sequencer::{
    interface::{client::*, operator::*, rollup::*, sequencer::*},
    task::registerer,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let env_variables: Vec<String> = env::args().skip(1).collect();

    // Initialize the listener socket.
    let address = env_variables
        .get(0)
        .wrap("Provide the sequencer address to bind to")?;
    let listener = TcpListener::bind(address)
        .await
        .wrap(format!("Failed to bind to {:?}", address))?;
    let sequencer_id: SequencerId = address.into();

    // Initialize the database.
    let database_path = env::current_dir()
        .wrap("Failed to get the current directory")?
        .join(format!("databases/ssal-sequencer/{}", sequencer_id));
    let database = Database::new(database_path)?;

    let rollup_id: RollupId = env_variables
        .get(1)
        .wrap("Provide the target rollup ID")?
        .as_str()
        .into();

    let ssal_url: Url = env_variables
        .get(2)
        .wrap("Provide SSAL URL to connect to")?
        .as_str()
        .try_into()
        .wrap("Failed to parse SSAL environment variable String into URL")?;

    // Init registerer task.
    registerer(
        database.clone(),
        ssal_url.clone(),
        rollup_id.clone(),
        sequencer_id.clone(),
    );

    // Set handlers
    let app = Router::new()
        .route("/client/send-transaction", post(SendTransaction::handler))
        .route(
            "/operator/block-commitment",
            get(GetBlockCommitment::handler),
        )
        .route("/rollup/block", get(GetBlock::handler))
        .route(
            "/sequencer/sync-transaction",
            post(SyncTransaction::handler),
        )
        .layer(CorsLayer::permissive())
        .with_state(database);

    // Start the sequencer.
    tracing::info!("Starting the server at {:?}", address);
    axum::serve(listener, app)
        .await
        .wrap("Failed to start the axum server")?;
    Ok(())
}
