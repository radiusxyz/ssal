use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
};
use ssal_core::error::{Error, WrapError};

abigen!(
    IIncredibleSquaringServiceManager,
    r"[
        function createNewTask(bytes calldata _commitment, uint32 _blockNumber, uint32 _rollupID, uint32 quorumThresholdPercentage, bytes calldata quorumNumbers) external
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
        .wrap("Failed to create a wallet")?;
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    Ok(client)
}
