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

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct DeregisterRollup {
    rollup_id: RollupId,
}

impl DeregisterRollup {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        // Deregister the rollup.
        let mut rollup_set: Lock<RollupSet> = state.get_mut(&"rollup_set")?;
        rollup_set.deregister(&payload.rollup_id)?;

        // Delete the block metadata associated with the rollup.
        state.delete(&("block_height", &payload.rollup_id))?;
        rollup_set.commit()?;

        tracing::info!("[DeregisterRollup]: {:?}", payload.rollup_id);
        Ok((StatusCode::OK, ()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct CloseBlock {
    rollup_id: RollupId,
}

impl CloseBlock {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        tracing::info!("[CloseBlock]: {:?}", payload.rollup_id);

        // Get the current block height.
        let mut block_height: Lock<BlockHeight> =
            state.get_mut(&("block_height", &payload.rollup_id))?;
        block_height.increment();
        let previous_block_height = block_height.clone() - 1;
        block_height.commit()?;

        // Always use the previous block height.
        // Elect the leader.
        let registered_sequencers_key =
            ("sequencer_set", &payload.rollup_id, &previous_block_height);
        let mut sequencer_set: Lock<SequencerSet> = state.get_mut(&registered_sequencers_key)?;
        let leader_id = sequencer_set.elect_leader()?;

        // Advertise the sequencer_set.
        state.put(
            &(
                "registered_sequencers",
                &payload.rollup_id,
                &previous_block_height,
            ),
            &*sequencer_set,
        )?;
        sequencer_set.commit()?;

        tracing::info!(
            "[CloseBlock]: Successfully elected the leader for {:?}: {:?}",
            payload.rollup_id,
            previous_block_height,
        );
        Ok((StatusCode::OK, Json(leader_id)))
    }
}
