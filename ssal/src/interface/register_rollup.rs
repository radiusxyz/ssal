use super::prelude::*;

/// Add RollupId in the RollupSet and inserts into Database an initial block with height 0.
/// The initial block returns nothing but signals sequencers that they can join in SequencerPool
/// for the block 1 of the corresponding rollup.
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct RegisterRollup {
    rollup_id: RollupId,
}

impl RegisterRollup {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        tracing::info!("[RegisterRollup]: {:?}", payload.rollup_id);

        // Register the rollup.
        let mut rollup_set: Lock<RollupSet> = state.get_mut(&"rollup_set")?;
        rollup_set.register(payload.rollup_id.clone())?;

        // Insert initial block metadata for the rollup.
        let initial_block = BlockHeight::from(1);
        state.put(&("block_height", &payload.rollup_id), &initial_block)?;
        rollup_set.commit()?;
        Ok((StatusCode::OK, ()))
    }
}
