use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct SyncTransaction {
    rollup_id: RollupId,
    raw_tx: RawTransaction,
}

impl SyncTransaction {
    pub async fn handler(
        State(state): State<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let mut block_metadata: Lock<BlockMetadata> = state
            .database()
            .get_mut(&("block_metadata", &payload.rollup_id))?;
        let block_height = block_metadata.block_height();
        let tx_order = block_metadata.issue_tx_order();

        state.database().put(
            &("raw_tx", &payload.rollup_id, &block_height, &tx_order),
            &payload.raw_tx,
        )?;

        block_metadata.commit()?;
        Ok((StatusCode::OK, ()))
    }
}
