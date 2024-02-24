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
        let initial_block = BlockHeight::from(0);
        state.put(&("block_height", &payload.rollup_id), &initial_block)?;
        rollup_set.commit()?;
        Ok((StatusCode::OK, ()).into_response())
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
        Ok((StatusCode::OK, ()).into_response())
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

        // Elect the leader.
        match Self::close_block(&state, block_height.clone(), &payload.rollup_id).await {
            Ok(leader) => {
                // Increment the block.
                block_height.increment();
                block_height.commit()?;
                Ok((StatusCode::OK, Json(leader)).into_response())
            }
            Err(error) => {
                // Increment the block.
                block_height.increment();
                block_height.commit()?;
                Err(error)
            }
        }
    }

    pub async fn close_block(
        database: &Database,
        block_height: BlockHeight,
        rollup_id: &RollupId,
    ) -> Result<SequencerId, Error> {
        // If the block height is 0, it means it is the initial block which has no
        // previous block. At this stage, sequencers are registering for the block 1.
        if block_height == 0 {
            Err(Error::from("The initial block has no leader."))
        } else {
            // Elect the leader.
            let mut sequencer_set: Lock<SequencerSet> =
                database.get_mut(&("sequencer_set", &rollup_id, &block_height))?;

            let leader = sequencer_set.elect_leader()?;

            // Publish the leader.
            database.put(&("leader", &rollup_id, &block_height), &leader)?;
            Ok(leader)
        }
    }
}
