use super::prelude::*;

#[derive(Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetSequencerSet {
    rollup_id: RollupId,
}

impl GetSequencerSet {
    pub async fn handler(
        State(state): State<Database>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let block_height: Lock<BlockHeight> =
            state.get_mut(&Key::BlockHeight(parameter.rollup_id.clone()))?;

        match block_height.value() {
            0 => Err(Error::from("Rollup registration in progress.")),
            1 => Err(Error::from("Initial block cannot be processed.")),
            _greater_than_or_equal_to_2 => {
                let previous_block_height = block_height.clone() - 1;
                drop(block_height);

                // Always use the previous block height.
                let sequencer_set: SequencerSet = state.get(&Key::SequencerSet(
                    parameter.rollup_id,
                    previous_block_height,
                ))?;
                Ok((StatusCode::OK, Json(sequencer_set)).into_response())
            }
        }
    }
}
