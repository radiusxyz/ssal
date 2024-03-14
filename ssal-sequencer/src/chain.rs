use std::{str::FromStr, sync::Arc};

use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
};
use ssal_core::{
    error::{Error, WrapError},
    types::*,
};

abigen!(
    IIncredibleSquaringTaskManager,
    r"[
        function createNewTask(bytes calldata commitment, uint32 blockNumber, uint32 rollupID, uint32 quorumThresholdPercentage, bytes calldata quorumNumbers) external
    ]"
);

pub async fn init_client(
    chain_url: impl AsRef<str>,
    private_key: impl AsRef<str>,
) -> Result<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, Error> {
    let provider =
        Provider::<Http>::try_from(chain_url.as_ref()).wrap("Failed to create a provider")?;
    let wallet: LocalWallet = private_key
        .as_ref()
        .parse::<LocalWallet>()
        .wrap("Failed to create a wallet")?
        .with_chain_id(Chain::AnvilHardhat);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    Ok(client)
}

pub async fn send_block_commitment(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    rollup_id: &RollupId,
    block_height: &BlockHeight,
    block_commitment: &Vec<u8>,
) -> Result<(), Error> {
    let contract_address = H160::from_str("0x84eA74d481Ee0A5332c457a4d796187F6Ba67fEB")
        .wrap("Failed to create a contract address")?;
    let contract = IIncredibleSquaringTaskManager::new(contract_address, client.clone());
    let block_commitment_bytes = Bytes::from_iter(block_commitment);
    let rollup_id_u32 = <RollupId as AsRef<str>>::as_ref(rollup_id)
        .parse::<u32>()
        .wrap("Failed to parse RollupId to u32")?;

    contract
        .create_new_task(
            block_commitment_bytes,
            block_height.value() as u32,
            rollup_id_u32,
            100,
            Bytes::from((0 as i32).to_be_bytes()),
        )
        .send()
        .await
        .wrap("Failed to create a new task")?;
    Ok(())
}
