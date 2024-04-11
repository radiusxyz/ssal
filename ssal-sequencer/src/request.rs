use std::{any, collections::HashMap, str::FromStr, time::Duration};

use ssal_core::{
    error::{Error, WrapError},
    reqwest::{Client, StatusCode, Url},
    types::*,
};

pub async fn register(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
    sequencer_id: &SequencerId,
) -> Result<Option<BlockHeight>, Error> {
    let url = ssal_base_url
        .join("register-sequencer")
        .wrap("[RegisterSequencer] Failed to parse into URL")?;

    let mut payload: HashMap<&'static str, String> = HashMap::new();
    payload.insert("rollup_id", rollup_id.to_string());
    payload.insert("sequencer_id", sequencer_id.to_string());

    let response = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .wrap("[RegisterSequencer]: Failed to send a request")?;

    if response.status() == StatusCode::OK {
        let block_height = response.json::<BlockHeight>().await.wrap(format!(
            "[RegisterSequencer]: Failed to parse the response into type: {}",
            any::type_name::<BlockHeight>(),
        ))?;
        Ok(Some(block_height))
    } else {
        Ok(None)
    }
}

pub async fn get_closed_sequencer_set(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
    block_height: &BlockHeight,
) -> Result<Option<SequencerSet>, Error> {
    let url = ssal_base_url
        .join("/get-closed-sequencer-set")
        .wrap("[GetClosedSequencerSet]: Failed to parse into URL")?;

    let query = [
        ("rollup_id", rollup_id.to_string()),
        ("block_height", block_height.to_string()),
    ];

    let response = Client::new()
        .get(url)
        .query(&query)
        .send()
        .await
        .wrap("[GetClosedSequencerSet]: Failed to send a request")?;

    if response.status() == StatusCode::OK {
        let sequencer_set = response.json::<SequencerSet>().await.wrap(format!(
            "[GetClosedSequencerSet]: Failed to parse the response into type: {}",
            any::type_name::<SequencerSet>()
        ))?;
        Ok(Some(sequencer_set))
    } else {
        Ok(None)
    }
}

pub async fn forward_transaction(
    leader_id: &SequencerId,
    rollup_id: &RollupId,
    raw_tx: &RawTransaction,
) -> Result<OrderCommitment, Error> {
    let url = Url::from_str(leader_id.as_ref())
        .wrap("[SendTransaction]: Failed to parse into URL (base)")?
        .join("/send-transaction")
        .wrap("[SendTransaction]: Failed to parse into URL (path)")?;

    let mut payload: HashMap<&'static str, String> = HashMap::new();
    payload.insert("rollup_id", rollup_id.to_string());
    payload.insert("raw_tx", raw_tx.to_string());

    let response = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .wrap("[SendTransaction]: Failed to send a request")?;

    if response.status() == StatusCode::OK {
        let order_commitment = response.json::<OrderCommitment>().await.wrap(format!(
            "[SendTransaction]: Failed to parse the response into type: {}",
            any::type_name::<OrderCommitment>(),
        ))?;
        Ok(order_commitment)
    } else {
        let error = response
            .text()
            .await
            .wrap("[SendTransaction]: Failed to parse the response into String")?;
        Err(Error::from(error))
    }
}

pub async fn sync_transaction(
    follower_id: SequencerId,
    leader_id: SequencerId,
    rollup_id: RollupId,
    raw_tx: RawTransaction,
) -> Result<(), Error> {
    let url = Url::from_str(follower_id.as_ref())
        .wrap("[SyncTransaction]: Failed to parse into URL (base)")?
        .join("/sync-transaction")
        .wrap("[SyncTransaction]: Failed to parse into URL (path)")?;

    let mut payload: HashMap<&'static str, String> = HashMap::new();
    // SSAL-009
    payload.insert("leader_id", leader_id.to_string());
    payload.insert("rollup_id", rollup_id.to_string());
    payload.insert("raw_tx", raw_tx.to_string());

    Client::new()
        .post(url)
        .timeout(Duration::from_secs(3))
        .json(&payload)
        .send()
        .await
        .wrap("[SyncTransaction]: Failed to send a request")?;
    Ok(())
}
