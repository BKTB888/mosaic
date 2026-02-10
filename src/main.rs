use std::net::SocketAddr;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use std::str::FromStr;
use serde::Serialize;
use axum::{
    extract::Path,
    response::Json,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use crate::runtime::system::storage::types::account::Account;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use tower_governor::key_extractor::SmartIpKeyExtractor;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 1. feladat
    let account_info = get_account_info().await?;

    println!("{:#?}", account_info);

    // 2. feladat

    //Bónusz
    let governor_conf = GovernorConfigBuilder::default()
        .const_per_second(60)
        .burst_size(1)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let app = Router::new()
        .route("/accounts/{address}", get(get_account))
        .layer(GovernorLayer::new(governor_conf));

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr)
        .await?;

    println!("Listening on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct AccountResponse {
    nonce: u32,
    consumers: u32,
    providers: u32,
    sufficients: u32,
    data: AccountData,
}

impl From<Account> for AccountResponse {
    fn from(account_info: Account) -> Self {
        AccountResponse {
            nonce: account_info.nonce,
            consumers: account_info.consumers,
            providers: account_info.providers,
            sufficients: account_info.sufficients,
            data: AccountData {
                free: account_info.data.free.to_string(),
                reserved: account_info.data.reserved.to_string(),
                frozen: account_info.data.frozen.to_string(),
                flags: format!("{:?}", account_info.data.flags),
            },
        }
    }
}

#[derive(Serialize)]
struct AccountData {
    free: String,
    reserved: String,
    frozen: String,
    flags: String,
}

async fn get_account_info() -> Result<Account, Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(
        "wss://testnet-gw1.mosaicchain.io/testnet-blockchain-1/chain"
    ).await?;

    let account =
        AccountId32::from_str("14s3KFN3AHnQ8xji3cd7BEMzF4ciipNRv3azgQwjFrf5seaW")?;

    Ok(
        api
        .storage()
        .at_latest()
        .await?
        .fetch(&runtime::storage().system().account(account))
        .await?
        .expect("Account not found")
    )
}

async fn get_account(Path(address): Path<String>) -> Json<AccountResponse> {
    // In real life you'd fetch this from a DB or chain state
    println!("Requested account: {}", address);

    let account_info = get_account_info().await.unwrap();

    Json(AccountResponse::from(account_info))
}
