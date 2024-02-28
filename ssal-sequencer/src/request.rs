use std::{any, collections::HashMap, str::FromStr};

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
        .join("sequencer/register")
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

pub async fn get_registered_sequencers(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
    block_height: &BlockHeight,
) -> Result<Option<SequencerSet>, Error> {
    let url = ssal_base_url
        .join("sequencer/registered-sequencers")
        .wrap("[GetRegisteredSequencers]: Failed to parse into URL")?;

    let query = [
        ("rollup_id", rollup_id.to_string()),
        ("block_height", block_height.to_string()),
    ];

    let response = Client::new()
        .get(url)
        .query(&query)
        .send()
        .await
        .wrap("[GetRegisteredSequencers]: Failed to send a request")?;

    if response.status() == StatusCode::OK {
        let registered_sequencers = response.json::<SequencerSet>().await.wrap(format!(
            "[GetRegisteredSequencers]: Failed to parse the response into type: {}",
            any::type_name::<SequencerSet>()
        ))?;
        Ok(Some(registered_sequencers))
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
        .join("/client/send-transaction")
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
    follower_id: &SequencerId,
    rollup_id: &RollupId,
    raw_tx: &RawTransaction,
) -> Result<(), Error> {
    let url = Url::from_str(follower_id.as_ref())
        .wrap("[SyncTransaction]: Failed to parse into URL (base)")?
        .join("/sequencer/sync-transaction")
        .wrap("[SyncTransaction]: Failed to parse into URL (path)")?;

    let mut payload: HashMap<&'static str, String> = HashMap::new();
    payload.insert("rollup_id", rollup_id.to_string());
    payload.insert("raw_tx", raw_tx.to_string());

    Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .wrap("[SyncTransaction]: Failed to send a request")?;
    Ok(())
}
