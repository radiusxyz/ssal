use std::{any, env};

use ssal_core::{
    error::{Error, WrapError},
    reqwest::{Client, Url},
    serde_json::json,
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
    let rollup_id: RollupId = env_variables
        .get(0)
        .wrap("Provide the rollup ID")?
        .as_str()
        .into();
    let ssal_url: Url = env_variables
        .get(1)
        .wrap("Provide SSAL URL to connect to")?
        .as_str()
        .try_into()
        .wrap("Failed to parse SSAL environment variable String into URL")?;

    register(&ssal_url, &rollup_id).await?;
    let mut block_height = 1;
    loop {
        sleep(Duration::from_secs(5)).await;
        if let Some(leader_id) = close_block(&ssal_url, &rollup_id).await? {
            tracing::info!("{:?}, {:?}, {:?}", rollup_id, block_height, leader_id);
            block_height += 1;
        }
    }
}

async fn register(ssal_base_url: &Url, rollup_id: &RollupId) -> Result<(), Error> {
    let url = ssal_base_url
        .join("register-rollup")
        .wrap("[RegisterRollup] Failed to parse into URL")?;

    let payload = json!({
        "rollup_id": rollup_id
    });

    let response = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .wrap("[RegisterRollup]: Failed to send a request")?;

    match response.error_for_status_ref() {
        Ok(_) => {
            tracing::info!("[RegisterRollup]: Successfully registered {:?}", rollup_id);
            Ok(())
        }
        Err(_) => {
            let error = response
                .text()
                .await
                .wrap("[RegisterRollup]: Failed to parse the response into String")?;
            tracing::error!("[RegisterRollup]: {}", error);
            Ok(())
        }
    }
}

async fn close_block(
    ssal_base_url: &Url,
    rollup_id: &RollupId,
) -> Result<Option<SequencerId>, Error> {
    let url = ssal_base_url
        .join("close-block")
        .wrap("[CloseBlock] Failed to parse into URL")?;

    let payload = json!({
        "rollup_id": rollup_id
    });

    let response = Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await
        .wrap("[CloseBlock]: Failed to send a request")?;

    match response.error_for_status_ref() {
        Ok(_) => {
            let leader_id = response.json::<SequencerId>().await.wrap(format!(
                "[CloseBlock]: Failed to parse the response into type: {}",
                any::type_name::<SequencerId>(),
            ))?;
            Ok(Some(leader_id))
        }
        Err(_) => {
            let error = response
                .text()
                .await
                .wrap("[CloseBlock]: Failed to parse the response into String")?;
            tracing::error!("[CloseBlock]: {}", error);
            Ok(None)
        }
    }
}
