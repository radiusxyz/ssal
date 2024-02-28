use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetBlockCommitment {
    rollup_id: RollupId,
    block_height: BlockHeight,
}

impl GetBlockCommitment {
    pub async fn handler(
        State(state): State<Database>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let block_commitment: String = state.get(&(
            "block_commitment",
            &parameter.rollup_id,
            &parameter.block_height,
        ))?;
        Ok((StatusCode::OK, Json(block_commitment)))
    }
}
