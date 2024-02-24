use std::env;

use ssal_core::{
    error::{Error, WrapError},
    reqwest::Url,
    tokio::{
        self,
        time::{sleep, Duration},
    },
    tracing_subscriber,
    types::*,
};
use ssal_database::Database;
use ssal_sequencer::task::registerer;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    let env_variables: Vec<String> = env::args().skip(1).collect();
    let rollup_id: RollupId = env_variables
        .get(0)
        .wrap("Provide the target rollup ID")?
        .as_str()
        .into();
    let sequencer_id: SequencerId = env_variables
        .get(1)
        .wrap("Provide the sequencer ID")?
        .as_str()
        .into();
    let ssal_url: Url = env_variables
        .get(2)
        .wrap("Provide SSAL URL to connect to")?
        .as_str()
        .try_into()
        .wrap("Failed to parse SSAL environment variable String into URL")?;

    // Initialize the database.
    let database_path = env::current_dir()
        .wrap("Failed to get the current directory")?
        .join(format!("databases/ssal-sequencer/{}", sequencer_id));
    let database = Database::new(database_path)?;

    // Init registerer task.
    registerer(
        database.clone(),
        ssal_url.clone(),
        rollup_id.clone(),
        sequencer_id.clone(),
    );

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

// pub async fn register(
//     ssal_base_url: &Url,
//     rollup_id: &RollupId,
//     sequencer_id: &SequencerId,
// ) -> Result<Option<BlockHeight>, Error> {
//     let url = ssal_base_url
//         .join("sequencer/register")
//         .wrap("[RegisterSequencer] Failed to parse into URL")?;

//     let mut payload: HashMap<&'static str, String> = HashMap::new();
//     payload.insert("rollup_id", rollup_id.to_string());
//     payload.insert("sequencer_id", sequencer_id.to_string());

//     let response = Client::new()
//         .post(url)
//         .json(&payload)
//         .send()
//         .await
//         .wrap("[RegisterSequencer]: Failed to send a request")?;

//     match response.error_for_status_ref() {
//         Ok(_) => {
//             let block_height: BlockHeight = response
//                 .text()
//                 .await
//                 .wrap("[RegisterSequencer]: Failed to parse the response into String")?
//                 .parse::<usize>()
//                 .wrap("[RegisterSequencer]: Failed to parse BlockHeight String into usize")?
//                 .into();
//             tracing::info!(
//                 "[RegisterSequencer]: Successfully registered for {:?}: {:?}",
//                 &rollup_id,
//                 &block_height,
//             );
//             Ok(Some(block_height))
//         }
//         Err(_) => {
//             let error = response
//                 .text()
//                 .await
//                 .wrap("[RegisterSequencer]: Failed to parse the response into String")?;
//             tracing::error!("[RegisterSequencer]: {}", error);
//             Ok(None)
//         }
//     }
// }

// pub async fn poll_leader(
//     ssal_base_url: &Url,
//     rollup_id: &RollupId,
//     block_height: &BlockHeight,
// ) -> Result<SequencerId, Error> {
//     let url = ssal_base_url
//         .join("sequencer/leader")
//         .wrap("[GetLeader] Failed to parse into URL")?;
//     let query = [
//         ("rollup_id", rollup_id.to_string()),
//         ("block_height", block_height.to_string()),
//     ];

//     loop {
//         sleep(Duration::from_millis(200)).await;
//         let response = Client::new()
//             .get(url.clone())
//             .query(&query)
//             .send()
//             .await
//             .wrap("[GetLeader]: Failed to send a request")?;

//         if response.status() == StatusCode::OK {
//             let leader_id: SequencerId = response
//                 .text()
//                 .await
//                 .wrap("[GetLeader] Failed to parse the response into String")?
//                 .into();
//             return Ok(leader_id);
//         }
//     }
// }
