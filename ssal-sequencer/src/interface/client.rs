use super::prelude::*;
use crate::request::{forward_transaction, sync_transaction};

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
        let mut block_metadata: Lock<BlockMetadata> =
            state.get_mut(&("block_metadata", &payload.rollup_id))?;

        if block_metadata.is_leader() {
            let leader_id = block_metadata.leader_id();
            let block_height = block_metadata.block_height();
            let tx_order = block_metadata.issue_tx_order();

            // Sync the transaction.
            let registered_sequencers: SequencerSet =
                state.get(&("registered_sequencers", &payload.rollup_id, &block_height))?;
            for follower_id in registered_sequencers.iter() {
                if *follower_id != leader_id {
                    let _ =
                        sync_transaction(follower_id, &payload.rollup_id, &payload.raw_tx).await;
                }
            }

            state.put(
                &("raw_tx", &payload.rollup_id, &block_height, &tx_order),
                &payload.raw_tx,
            )?;
            block_metadata.commit()?;

            // Return the order commitment.
            let order_commitment = OrderCommitment::new(block_height, tx_order);
            Ok((StatusCode::OK, Json(order_commitment)))
        } else {
            let leader_id = block_metadata.leader_id();
            drop(block_metadata);

            let order_commitment =
                forward_transaction(&leader_id, &payload.rollup_id, &payload.raw_tx).await?;
            Ok((StatusCode::OK, Json(order_commitment)))
        }
    }
}
