use std::sync::Arc;

use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
};
use ssal_database::Database;

pub struct AppState {
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    database: Database,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            database: self.database.clone(),
        }
    }
}

impl AppState {
    pub fn new(
        client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        database: Database,
    ) -> Self {
        Self {
            client: Arc::new(client),
            database,
        }
    }

    pub fn client(&self) -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
        self.client.clone()
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}
