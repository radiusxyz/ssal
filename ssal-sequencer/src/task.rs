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
                    block_height,
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
                        block_metadata.update(block_height, leader_id == sequencer_id, leader_id);
                        block_metadata.commit().unwrap();
                    }
                    Err(error) => {
                        if error.is_none_type() {
                            let block_metadata = BlockMetadata::new(
                                block_height,
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
