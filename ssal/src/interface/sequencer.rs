use ssal_core::{
    axum::{extract::State, response::IntoResponse, Json},
    tracing,
};
use ssal_database::Database;

pub struct Register {}

impl Register {
    pub async fn handler(
        Json(payload): Json<Self>,
        State(state): State<Database>,
    ) -> impl IntoResponse {
        tracing::info!("Sequencer registered");
    }
}
