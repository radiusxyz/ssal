use std::sync::Arc;

use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
};
use ssal_database::Database;

pub struct AppState(Arc<AppStateInner>);

pub struct AppStateInner {
    client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    database: Database,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl AppState {
    pub fn new(
        client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        database: Database,
    ) -> Self {
        Self(Arc::new(AppStateInner { client, database }))
    }

    pub fn client(&self) -> &SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
        &self.0.client
    }

    pub fn database(&self) -> &Database {
        &self.0.database
    }
}
