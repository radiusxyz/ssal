use super::prelude::*;

#[derive(Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct RegisterSequencer {
    rollup_id: RollupId,
    sequencer_id: SequencerId,
}

impl RegisterSequencer {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let block_height: Lock<BlockHeight> =
            state.get_mut(&Key::BlockHeight(payload.rollup_id.clone()))?;
        let next_block_height = block_height.clone() + 1;

        // Always register for the next block.
        let mut sequencer_set: Lock<SequencerSet> = state.get_mut(&Key::SequencerSet(
            payload.rollup_id,
            next_block_height.clone(),
        ))?;
        sequencer_set.register(payload.sequencer_id)?;
        Ok((StatusCode::OK, Json(next_block_height)).into_response())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetLeader {
    rollup_id: RollupId,
}

impl GetLeader {
    pub async fn handler(
        State(state): State<Database>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let block_height: Lock<BlockHeight> =
            state.get_mut(&Key::BlockHeight(parameter.rollup_id.clone()))?;
        let previous_block_height = block_height.clone() - 1;
        drop(block_height);

        // Always use the previous block height.
        let leader: SequencerId =
            state.get(&Key::Leader(parameter.rollup_id, previous_block_height))?;
        Ok((StatusCode::OK, Json(leader)).into_response())
    }
}
