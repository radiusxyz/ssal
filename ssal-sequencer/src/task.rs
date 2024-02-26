use ssal_core::{
    reqwest::Url,
    tokio::{
        self,
        time::{sleep, Duration},
    },
    tracing,
    types::*,
};
use ssal_database::Database;

use crate::request::{get_leader, register};

pub fn registerer(
    database: Database,
    ssal_url: Url,
    rollup_id: RollupId,
    sequencer_id: SequencerId,
) {
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
                    database.clone(),
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
    database: Database,
    ssal_url: Url,
    rollup_id: RollupId,
    sequencer_id: SequencerId,
    block_height: BlockHeight,
) {
    tokio::spawn(async move {
        loop {
            if let Some(leader_id) = get_leader(&ssal_url, &rollup_id, &block_height)
                .await
                .unwrap()
            {
                let block_metadata_key = ("block_metadata", &rollup_id);
                match database
                    .get_mut::<(&'static str, &RollupId), BlockMetadata>(&block_metadata_key)
                {
                    Ok(mut block_metadata) => {
                        if block_metadata.is_leader() {
                            let current_block_height = block_metadata.block_height();
                            let current_tx_order = block_metadata.tx_order();
                            block_builder(
                                database.clone(),
                                rollup_id,
                                current_block_height,
                                current_tx_order,
                            );
                        }

                        block_metadata.update(
                            block_height.clone(),
                            leader_id == sequencer_id,
                            leader_id,
                        );
                        block_metadata.commit().unwrap();
                    }
                    Err(error) => {
                        if error.is_none_type() {
                            let block_metadata = BlockMetadata::new(
                                block_height.clone(),
                                leader_id == sequencer_id,
                                leader_id,
                            );
                            database.put(&block_metadata_key, &block_metadata).unwrap();
                        }
                    }
                }
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
    });
}

pub fn block_builder(
    database: Database,
    rollup_id: RollupId,
    block_height: BlockHeight,
    tx_count: TransactionOrder,
) {
    tokio::spawn(async move {
        let block: Vec<RawTransaction> = tx_count
            .iter()
            .map(|tx_order| {
                let raw_tx: RawTransaction = database
                    .get(&("raw_tx", &rollup_id, &block_height, &tx_order))
                    .unwrap();
                raw_tx
            })
            .collect();

        database
            .put(&("block", &rollup_id, &block_height), &block)
            .unwrap();

        let commitment = ssal_commitment::get_block_commitment(block);
    });
}
