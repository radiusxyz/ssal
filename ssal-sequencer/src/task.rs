use ssal_core::{
    reqwest::Url,
    tokio::{
        self,
        time::{sleep, Duration},
    },
    tracing,
    types::*,
};

use crate::{
    app_state::AppState,
    chain::send_block_commitment,
    request::{get_closed_sequencer_set, register},
};

pub fn registerer(state: AppState, ssal_url: Url, rollup_id: RollupId, sequencer_id: SequencerId) {
    tokio::spawn(async move {
        loop {
            if let Some(block_height) = register(&ssal_url, &rollup_id, &sequencer_id)
                .await
                .unwrap()
            {
                tracing::info!(
                    "[RegisterSequencer]: Successfully registered for {:?}: {:?}",
                    &rollup_id,
                    &block_height,
                );
                leader_poller(
                    state.clone(),
                    ssal_url.clone(),
                    rollup_id.clone(),
                    sequencer_id.clone(),
                    block_height.clone(),
                );
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
}

pub fn leader_poller(
    state: AppState,
    ssal_url: Url,
    rollup_id: RollupId,
    sequencer_id: SequencerId,
    block_height: BlockHeight,
) {
    tokio::spawn(async move {
        loop {
            if let Some(sequencer_set) =
                get_closed_sequencer_set(&ssal_url, &rollup_id, &block_height)
                    .await
                    .unwrap()
            {
                let database = state.database();
                let leader_id = sequencer_set.leader().unwrap();

                // Store the sequencer set.
                let sequencer_set_key = ("sequencer_set", &rollup_id, &block_height);
                database.put(&sequencer_set_key, &sequencer_set).unwrap();

                // Store the block metadata.
                let block_metadata_key = ("block_metadata", &rollup_id, &block_height);
                let block_metadata = BlockMetadata::new(leader_id == sequencer_id);
                database.put(&block_metadata_key, &block_metadata).unwrap();

                if block_height.value() >= 2 {
                    block_builder(state.clone(), rollup_id.clone(), block_height.clone() - 1);
                }
                break;
            }

            sleep(Duration::from_millis(100)).await;
        }
    });
}

pub fn block_builder(state: AppState, rollup_id: RollupId, block_height: BlockHeight) {
    tokio::spawn(async move {});
}

// pub fn block_builder(
//     state: AppState,
//     rollup_id: RollupId,
//     block_height: BlockHeight,
//     tx_count: TransactionOrder,
//     is_leader: bool,
//     seed: [u8; 32],
// ) {
//     tokio::spawn(async move {
//         let block: Vec<RawTransaction> = tx_count
//             .iter()
//             .map(|tx_order| {
//                 let raw_tx: RawTransaction = state
//                     .database()
//                     .get(&("raw_tx", &rollup_id, &block_height, &tx_order))
//                     .unwrap();
//                 raw_tx
//             })
//             .collect();
//         state
//             .database()
//             .put(&("block", &rollup_id, &block_height), &block)
//             .unwrap();

//         let block_commitment = ssal_commitment::get_block_commitment(block, seed);
//         state
//             .database()
//             .put(
//                 &("block_commitment", &rollup_id, &block_height),
//                 &block_commitment,
//             )
//             .unwrap();

//         if is_leader {
//             send_block_commitment(state.client(), &rollup_id, &block_height, &block_commitment)
//                 .await
//                 .unwrap();

//             tracing::info!(
//                 "[Leader]: Successfully sent block commitment to the contract for {:?}: {:?}",
//                 rollup_id,
//                 block_height,
//             );
//         }
//     });
// }
