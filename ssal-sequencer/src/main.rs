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
use ssal_sequencer::{app_state::AppState, chain::init_client, interface::*, task::registerer};

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

    let chain_url = env_variables
        .get(3)
        .wrap("Provide the chain URL")?
        .to_string();
    let wallet_private_key = env_variables
        .get(4)
        .wrap("Provide the private key for the wallet")?;
    let client = init_client(&chain_url, &wallet_private_key).await?;

    let app_state = AppState::new(client, database);

    // Init registerer task.
    registerer(
        app_state.clone(),
        ssal_url.clone(),
        rollup_id.clone(),
        sequencer_id.clone(),
    );

    // Set handlers
    let app = Router::new()
        .route("/get-block-commitment", get(GetBlockCommitment::handler))
        .route("/get-block", get(GetBlock::handler))
        .route("/send-transaction", post(SendTransaction::handler))
        .route("/sync-transaction", post(SyncTransaction::handler))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start the sequencer.
    tracing::info!("Starting the server at {:?}", address);
    axum::serve(listener, app)
        .await
        .wrap("Failed to start the axum server")?;
    Ok(())
}
