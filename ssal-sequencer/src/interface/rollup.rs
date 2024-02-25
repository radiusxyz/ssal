use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetBlock {
    rollup_id: RollupId,
    block_height: BlockHeight,
}

impl GetBlock {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let block: Block = state.get(&("block", &payload.rollup_id, &payload.block_height))?;
        Ok((StatusCode::OK, Json(block)))
    }
}
