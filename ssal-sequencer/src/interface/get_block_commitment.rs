use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct GetBlockCommitment {
    rollup_id: RollupId,
    block_height: BlockHeight,
}

impl GetBlockCommitment {
    pub async fn handler(
        State(state): State<AppState>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        tracing::info!(
            "[Follower]: Operator retrieved the block commitment for {:?}: {:?}",
            &parameter.rollup_id,
            &parameter.block_height,
        );
        let block_commitment: String = state.database().get(&(
            "block_commitment",
            &parameter.rollup_id,
            &parameter.block_height,
        ))?;
        Ok((StatusCode::OK, block_commitment))
    }
}
