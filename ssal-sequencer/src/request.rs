use std::collections::HashMap;

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
        let block_height: BlockHeight = response
            .text()
            .await
            .wrap("[RegisterSequencer]: Failed to parse the response into String")?
            .parse::<usize>()
            .wrap("[RegisterSequencer]: Failed to parse BlockHeight String into usize")?
            .into();
        return Ok(Some(block_height));
    } else {
        return Ok(None);
    }
}

pub async fn get_leader(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
    block_height: &BlockHeight,
) -> Result<Option<SequencerId>, Error> {
    let url = ssal_base_url
        .join("sequencer/leader")
        .wrap("[GetLeader] Failed to parse into URL")?;

    let query = [
        ("rollup_id", rollup_id.to_string()),
        ("block_height", block_height.to_string()),
    ];

    let response = Client::new()
        .get(url.clone())
        .query(&query)
        .send()
        .await
        .wrap("[GetLeader]: Failed to send a request")?;

    if response.status() == StatusCode::OK {
        let leader_id: SequencerId = response
            .text()
            .await
            .wrap("[GetLeader]: Failed to parse the response into String")?
            .into();
        return Ok(Some(leader_id));
    } else {
        return Ok(None);
    }
}
