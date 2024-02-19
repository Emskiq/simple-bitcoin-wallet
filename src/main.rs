extern crate dotenv;
extern crate bdk;

use std::env;
use std::path::Path;

use bdk::{Wallet, SyncOptions};
use bdk::bitcoin::Network;
use bdk::database::SqliteDatabase;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::wallet::AddressIndex::New;

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

use serde;

fn setup () -> String {
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    match env::var("WALLET_DESCRIPTOR") {
        Err(_) => "error".to_string(),
        Ok(name) => name
    }
}

struct AppError(anyhow::Error);

#[derive(serde::Serialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

// anyhow -> handling of generic types of error and so on...
#[tokio::main] // This means that we "decorate" the function with the tokio and axum asunc
               // executor. This way we don't need to think about the locking and unlocking the
               // threads and so on...
async fn main() -> anyhow::Result<()> {
    let descriptor = setup();

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    let path : &Path = Path::new ("emskiq.db");

    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        SqliteDatabase::new(path))?;

    wallet.sync(&blockchain, SyncOptions::default())?;
    println!("Descriptor balance: {} SAT", wallet.get_balance()?);

    let address = wallet.get_address(New)?;
    dbg!(address);

    let address = wallet.get_address(New)?;
    dbg!(address);

    println!("Address #0: {}", wallet.get_address(New)?);
    println!("Address #1: {}", wallet.get_address(New)?);

    ///// AXUM Part
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    ///// AXUM

    Ok(())
}

async fn handler() -> Result<impl IntoResponse, AppError> {
    let response = AddressResponse {
        address: "test".to_string(),
        index: 0,
    };
    Ok (Json(response))
}

// Tell axum how to convert our custom AppError into a response
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Sth si eba maikata: {}", self.0), // since the first (and only element of the
                                                       // tupple wrapper AppError is our
                                                       // anyhow::Erorr
        ).into_response()
    }
}

impl<E : Into<anyhow::Error>> From<E> for AppError
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
