use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetBlock {
    rollup_id: RollupId,
    block_height: BlockHeight,
}

impl GetBlock {
    pub async fn handler(
        State(state): State<AppState>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let block: Vec<RawTransaction> =
            state
                .database()
                .get(&("block", &parameter.rollup_id, &parameter.block_height))?;
        Ok((StatusCode::OK, Json(block)))
    }
}
