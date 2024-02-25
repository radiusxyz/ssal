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
        .get(1)
        .wrap("Provide SSAL URL to connect to")?
        .as_str()
        .try_into()
        .wrap("Failed to parse SSAL environment variable String into URL")?;
    let rollup_id: RollupId = env_variables.get(2).wrap("Provide the rollup ID")?.into();

    loop {
        if let Some(sequencer_set) = get_sequencer_set(&ssal_url, &rollup_id).await? {}
        sleep(Duration::from_millis(100)).await;
    }
}

pub async fn get_sequencer_set(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
) -> Result<Option<SequencerSet>, Error> {
    let url = ssal_base_url
        .join("client/sequencer-set")
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

pub fn send_transaction() {}
