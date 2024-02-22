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
        // Register the rollup.
        let mut rollup_set: Lock<RollupSet> = state.get_mut(&Key::RollupSet)?;
        rollup_set.register(payload.rollup_id.clone())?;

        // Insert initial block metadata for the rollup.
        let initial_block = BlockHeight::from(0);
        state.put(&Key::BlockHeight(payload.rollup_id), &initial_block)?;
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
        let mut rollup_set: Lock<RollupSet> = state.get_mut(&Key::RollupSet)?;
        rollup_set.deregister(&payload.rollup_id)?;

        // Delete the block metadata associated with the rollup.
        state.delete(&Key::BlockHeight(payload.rollup_id))?;
        rollup_set.commit()?;
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
        // Get the current block height.
        let mut block_height: Lock<BlockHeight> =
            state.get_mut(&Key::BlockHeight(payload.rollup_id.clone()))?;

        // If the block height is 0, it means it is the initial block which has no
        // previous block. At this stage, sequencers are registering for the block 1.
        if *block_height == 0 {
            block_height.increment();
            block_height.commit()?;
            return Err(Error::from("The initial block has no leader."));
        } else {
            // Elect the leader.
            let mut sequencer_set: Lock<SequencerSet> = state.get_mut(&Key::SequencerSet(
                payload.rollup_id.clone(),
                block_height.clone(),
            ))?;
            let leader = sequencer_set.elect_leader()?;

            // Publish the leader.
            state.put(
                &Key::Leader(payload.rollup_id, block_height.clone()),
                &leader,
            )?;

            // Increment the block.
            block_height.increment();
            block_height.commit()?;
            Ok((StatusCode::OK, Json(leader)).into_response())
        }
    }
}
