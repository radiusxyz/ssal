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
        let block_height_key = ("block_height", &payload.rollup_id);
        let block_height: Lock<BlockHeight> = state.get_mut(&block_height_key)?;

        let sequencer_set_key = ("registered_sequencers", &payload.rollup_id, &*block_height);
        match state.get_mut::<(&str, &RollupId, &BlockHeight), SequencerSet>(&sequencer_set_key) {
            Ok(mut sequencer_set) => {
                sequencer_set.register(payload.sequencer_id)?;
                sequencer_set.commit()?;
                Ok((StatusCode::OK, Json(block_height.clone())))
            }
            Err(error) => match error.is_none_type() {
                true => {
                    let mut sequencer_set = SequencerSet::new(block_height.clone());
                    sequencer_set.register(payload.sequencer_id)?;
                    state.put(&sequencer_set_key, &sequencer_set)?;
                    Ok((StatusCode::OK, Json(block_height.clone())))
                }
                false => Err(error),
            },
        }
    }
}
