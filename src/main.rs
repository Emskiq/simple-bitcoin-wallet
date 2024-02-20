extern crate dotenv;
extern crate bdk;

mod app_state;
mod bip21;

use std::{env, sync::{Arc, RwLock}};

use axum::{extract::State, http::{self, StatusCode, Method}, Json, response::IntoResponse, routing::get, Router};
use bdk::{wallet::AddressIndex, bitcoin::Amount};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

use app_state::{AppState, SharedState};
use bip21::create_bip_21;

#[derive(Serialize)]
struct AddressResponse {
    address: String,
    bolt11: String,
    index: u32,
    bip21: String,
}

// anyhow -> handling of generic types of error and so on...
#[tokio::main] // This means that we "decorate" the function with the tokio and axum asunc
               // executor. This way we don't need to think about the locking and unlocking the
               // threads and so on...
async fn main() -> anyhow::Result<()> {
    // Load environment variables from various sources (only .env file rn)
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    // - Connection arguments which are use to connect to a lighting
    //   network so we can generate addresses and so on based on them
    // - For the lighting node we are choosing voltage
    // - You need to create a test LND Node though the Voltage website
    // - After the creation copy the node details:
    // - Macaroon and tls certificate: the connection pannel
    // - LND adrress -> API Endpoint address, which is in the Dashboard page
    // - other 3 environments are covered in the video
    let descriptor = env::var("WALLET_DESCRIPTOR")?;
    let network = env::var("NETWORK")?;
    let db_path = env::var("DB_PATH")?;
    let lnd_address = env::var("LND_ADDRESS")?;
    let mac_path = env::var("LND_MACAROON_PATH")?;
    let tls_path = env::var("LND_TLS_CERT_PATH")?;

    // Switched to fedimint_tonic_lnd package, because tonic_lnd wasn't
    // processing correctly the tls.cert file
    let client = fedimint_tonic_lnd::connect(lnd_address, tls_path, mac_path)
        .await
        .expect("failed to connect")
        .lightning()
        .clone();

    let state = AppState::new(descriptor, network, db_path, client)?;
    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // AXUM Part (or Server part)
    // Layer for the server to use
    let layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(vec![http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    // the server/app creation with its endpoints
    // build our application with a route
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/address", get(new_address_handler))
        .with_state(shared_state)
        .layer(layer);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_handler () -> &'static str {
    "hello!"
}

// The main endpoint that website is listening for
// this callback function is executed on every call
// from the front end
#[axum::debug_handler]
async fn new_address_handler(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    // Onchain
    let address = &state
        .read()
        .unwrap()
        .wallet
        .get_address(AddressIndex::New)?;

    dbg!(address);

    // Lightning
    // Hard thing is to use this mutable "Client" thing without Rust getting mad
    // Can't use these on the same line because Rust \o/
    // *** Again problems with the certificates reading...
    // let mut client = state.read().unwrap().lnd_client.clone();
    // let invoice = fedimint_tonic_lnd::lnrpc::Invoice {value: 1000, ..Default::default()};

    // let bolt11 = client
    //     .add_invoice(invoice)
    //     .await
    //     .unwrap()
    //     .into_inner()
    //     .payment_request;
    // dbg!(bolt11.clone());

    // Since the above code gives some errors regarding tls certificate count again...
    // I am adding some default value for the invoice string
    let bolt11 = "lnbc2500u1pvjluezpp5qqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqypqdq5xysxxatsyp3k7enxv4jsxqzpuaztrnwngzn3kdzw5hydlzf03qdgm2hdq27cqv3agm2awhz5se903vruatfhq77w3ls4evs3ch9zw97j25emudupq63nyw24cg27h2rspfj9srp".to_string();

    let bip21 = create_bip_21(
        address.address.clone(),
        bolt11.clone(),
        Amount::from_sat(1000),
        "heyo".to_string(),
    );

    let address_response = AddressResponse {
        address: address.address.to_string(),
        index: address.index,
        bolt11,
        bip21,
    };

    Ok(Json(address_response))
}

// Our own wrapper implementation of Error so we can handle it in the amux server
struct AppError(anyhow::Error);

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

// Implementation of the trait Into for our own AppError
impl<E : Into<anyhow::Error>> From<E> for AppError
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
