use std::{any, collections::HashMap, env, str::FromStr};

use ssal_core::{
    error::{Error, WrapError},
    reqwest::{Client, StatusCode, Url},
    tokio::{
        self,
        time::{sleep, Duration},
    },
    tracing, tracing_subscriber,
    types::*,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let env_variables: Vec<String> = env::args().skip(1).collect();
    let ssal_url: Url = env_variables
        .get(0)
        .wrap("Provide SSAL URL to connect to")?
        .as_str()
        .try_into()
        .wrap("Failed to parse SSAL environment variable String into URL")?;
    let rollup_id: RollupId = env_variables.get(1).wrap("Provide the rollup ID")?.into();

    let mut raw_tx_count = 0;
    loop {
        if let Some(mut sequencer_set) = get_sequencer_set(&ssal_url, &rollup_id).await? {
            // Using elect leader for a convenient random selection.
            let follower_id = sequencer_set.elect_leader()?;
            let block_height = sequencer_set.block_height();
            let raw_tx = RawTransaction::from(raw_tx_count.to_string());
            let order_commitment =
                send_transaction(follower_id, &rollup_id, &block_height, raw_tx).await?;
            tracing::info!("{:?}", order_commitment);
            raw_tx_count += 1;
        }
        sleep(Duration::from_millis(200)).await;
    }
}

pub async fn get_sequencer_set(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
) -> Result<Option<SequencerSet>, Error> {
    let url = ssal_base_url
        .join("get-sequencer-set")
        .wrap("[GetSequencerSet]: Failed to parse into URL")?;

    let query = [("rollup_id", rollup_id.to_string())];

    let response = Client::new()
        .get(url)
        .query(&query)
        .send()
        .await
        .wrap("[GetSequencerSet]: Failed to send a request")?;

    if response.status() == StatusCode::OK {
        let sequencer_set = response.json::<SequencerSet>().await.wrap(format!(
            "[GetSequencerSet]: Failed to parse the response into type: {}",
            any::type_name::<SequencerSet>(),
        ))?;
        Ok(Some(sequencer_set))
    } else {
        let error = response
            .text()
            .await
            .wrap("[GetSequencerSet]: Failed to parse the response into String")?;
        tracing::error!("{}", error);
        Ok(None)
    }
}

pub async fn send_transaction(
    sequencer_id: SequencerId,
    rollup_id: &RollupId,
    block_height: &BlockHeight,
    raw_tx: RawTransaction,
) -> Result<Option<OrderCommitment>, Error> {
    let url = Url::from_str(sequencer_id.as_ref())
        .wrap("[SendTransaction]: Failed to parse into URL (base)")?
        .join("send-transaction")
        .wrap("[SendTransaction]: Failed to parse into URL (path)")?;

    let mut payload: HashMap<&'static str, String> = HashMap::new();
    payload.insert("rollup_id", rollup_id.to_string());
    payload.insert("block_height", block_height.to_string());
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
        Ok(Some(order_commitment))
    } else {
        let error = response
            .text()
            .await
            .wrap("[SendTransaction]: Failed to parse the response into String")?;
        tracing::error!("{}", error);
        Ok(None)
    }
}
