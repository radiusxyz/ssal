use std::env;

use ssal_core::{
    axum::{
        self,
        routing::{get, post},
        Router,
    },
    error::{Error, WrapError},
    public_ip,
    reqwest::Url,
    tokio::{self, net::TcpListener},
    tower_http::cors::CorsLayer,
    tracing, tracing_subscriber,
    types::*,
};
use ssal_database::Database;
use ssal_sequencer::{
    app_state::AppState, chain::init_client, config::Config, interface::*, task::registerer,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    // SSAL-004
    let config_path = env::current_dir()
        .wrap("Failed to get the current directory")?
        .join("configs/ssal-sequencer.toml");
    tracing::info!("{:?}", config_path);
    let config = Config::from_path(config_path)?;
    let ssal_url: Url = config
        .ssal_url
        .as_str()
        .try_into()
        .wrap("Failed to parse SSAL environment variable String into URL")?;
    let rollup_id: RollupId = config.rollup_id.as_str().into();
    let chain_url = config.chain_url;
    let wallet_private_key = config.wallet_private_key;
    let client = init_client(&chain_url, &wallet_private_key).await?;

    // Initialize the listener socket.
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .wrap("Failed to bind to 0.0.0.0:8000")?;

    let sequencer_id = match config.is_local_deployment {
        true => SequencerId::from("0.0.0.0:8000"),
        false => {
            // SSAL-006
            // Assuming the port-forwarded setup
            let mut public_address = public_ip::addr()
                .await
                .wrap("Failed to get the public IP")?
                .to_string();
            public_address.push_str(":8000");
            SequencerId::from(&public_address)
        }
    };
    tracing::info!("Sequencer ID: {}", sequencer_id);

    // Initialize the database.
    let database_path = env::current_dir()
        .wrap("Failed to get the current directory")?
        .join(format!("databases/ssal-sequencer/{}", sequencer_id));
    let database = Database::new(database_path)?;

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
    tracing::info!("Starting the server at {:?}", sequencer_id);
    axum::serve(listener, app)
        .await
        .wrap("Failed to start the axum server")?;
    Ok(())
}
