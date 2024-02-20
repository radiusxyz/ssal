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
        let block_metadata: Lock<BlockMetadata> =
            state.get_mut(&Key::BlockMetadata(payload.rollup_id.clone()))?;

        let mut sequencer_pool: Lock<SequencerPool> = state.get_mut(&Key::SequencerPool(
            payload.rollup_id,
            block_metadata.get_height(),
        ))?;
        sequencer_pool.add(payload.sequencer_id)?;
        Ok((StatusCode::OK, ()).into_response())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetLeader {
    leader_id: SequencerId,
}

impl GetLeader {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<RegisterSequencer>,
    ) -> Result<(), Error> {
        // let mut leader: SequencerId = state.get()
        Ok(())
    }
}
