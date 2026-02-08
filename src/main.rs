use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use std::str::FromStr;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(
        "wss://testnet-gw1.mosaicchain.io/testnet-blockchain-1/chain"
    ).await?;

    let account =
        AccountId32::from_str("14s3KFN3AHnQ8xji3cd7BEMzF4ciipNRv3azgQwjFrf5seaW")?;


    /// 1. Feladat
    let account_info = api
        .storage()
        .at_latest()
        .await?
        .fetch(&runtime::storage().system().account(account))
        .await?
        .expect("Account not found");

    println!("{:#?}", account_info.data);

    Ok(())
}
