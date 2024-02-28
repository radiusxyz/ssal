use super::prelude::*;

#[derive(Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetClosedSequencerSet {
    rollup_id: RollupId,
    block_height: BlockHeight,
}

impl GetClosedSequencerSet {
    pub async fn handler(
        State(state): State<Database>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let closed_sequencer_set: SequencerSet = state.get(&(
            "closed_sequencer_set",
            &parameter.rollup_id,
            &parameter.block_height,
        ))?;
        Ok((StatusCode::OK, Json(closed_sequencer_set)))
    }
}
