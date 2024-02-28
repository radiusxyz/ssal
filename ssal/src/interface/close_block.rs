use super::prelude::*;

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
                "closed_sequencer_set",
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
