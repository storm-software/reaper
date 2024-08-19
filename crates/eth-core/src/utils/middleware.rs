use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::env;

use crate::constants::env::WALLET_PRIVATE_KEY;

/// Sets up middleware w/ our private key env var.
///
/// # Arguments
/// * `provider` - The provider to use.
///
/// # Returns
/// The signer middleware.
///
/// # Example
/// ```
/// let provider = Provider::<Http>::new("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID".into());
/// let signer = setup_signer_middleware(provider).await;
/// ```
///
pub async fn setup_signer_middleware(
  provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
  let chain_id = provider.get_chainid().await.expect("Failed to get chain id.");

  let private_key = env::var(WALLET_PRIVATE_KEY).expect("The `WALLET_PRIVATE_KEY` must be set");

  let wallet = private_key
    .parse::<LocalWallet>()
    .expect("Failed to parse wallet")
    .with_chain_id(chain_id.as_u64());

  SignerMiddleware::new(provider, wallet)
}
