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
        let block_metadata_key = Key::BlockMetadata(payload.rollup_id);

        match state.get_mut::<Key, BlockMetadata>(&block_metadata_key) {
            Ok(mut block_metadata) => {
                block_metadata.close_block();
                Ok(((StatusCode::OK, ())).into_response())
            }
            Err(error) => match error.is_none_type() {
                true => {
                    let new_block_metadata = BlockMetadata::default();
                    state.put(&block_metadata_key, &new_block_metadata)?;
                    return Ok(((StatusCode::OK, ())).into_response());
                }
                false => Err(error),
            },
        }
    }
}
