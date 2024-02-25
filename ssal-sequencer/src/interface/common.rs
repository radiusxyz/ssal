use super::prelude::*;
use crate::request::forward_transaction;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct SendTransaction {
    rollup_id: RollupId,
    raw_tx: RawTransaction,
}

impl SendTransaction {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        // let mut block_metadata: Lock<BlockMetadata> =
        //     state.get_mut(&("block_metadata", &payload.rollup_id))?;

        // if block_metadata.is_leader() {
        //     let block_height = block_metadata.block_height();
        //     let tx_order = block_metadata.issue_tx_order();
        //     state.put(
        //         &("raw_tx", &payload.rollup_id, &block_height, &tx_order),
        //         &payload.raw_tx,
        //     )?;
        //     block_metadata.commit()?;
        //     Ok((StatusCode::OK, Json(order_commitment)))
        // } else {
        //     let leader_id = block_metadata.leader_id();
        //     drop(block_metadata);
        //     let order_commitment =
        //         forward_transaction(leader_id, payload.rollup_id, payload.raw_tx).await?;
        //     Ok((StatusCode::OK, Json(order_commitment)))
        // }
        Ok((StatusCode::OK, ()))
    }
}
