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
        State(state): State<AppState>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        let mut block_metadata: Lock<BlockMetadata> = state
            .database()
            .get_mut(&("block_metadata", &payload.rollup_id))?;

        if block_metadata.is_leader() {
            let leader_id = block_metadata.leader_id();
            let block_height = block_metadata.block_height();

            // SSAL-002
            if block_metadata.tx_count() == TransactionOrder::from(128) {
                return Err(Error::from(
                    "Cannot include more transactions in the current block",
                ));
            }

            // Sync the transaction.
            let sequencer_set: SequencerSet =
                state
                    .database()
                    .get(&("sequencer_set", &payload.rollup_id, &block_height))?;

            // SSAL-009 & SSAL-010
            let handles: Vec<JoinHandle<Result<(), Error>>> = sequencer_set
                .iter()
                .map(|follower_id| {
                    tokio::spawn(sync_transaction(
                        follower_id.clone(),
                        leader_id.clone(),
                        payload.rollup_id.clone(),
                        payload.raw_tx.clone(),
                    ))
                })
                .collect();

            for handle in handles {
                if let Err(error) = handle.await {
                    tracing::error!("{}", error);
                }
            }

            let tx_order = block_metadata.issue_tx_order();
            state.database().put(
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
