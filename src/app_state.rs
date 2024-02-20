use std::{env, sync::{Arc, RwLock}};

use bdk::{Wallet, bitcoin::Network, database::SqliteDatabase};
use fedimint_tonic_lnd::LightningClient;

pub struct AppState {
    pub wallet: Wallet<SqliteDatabase>,
    pub lnd_client: LightningClient,
}

// lololol - unsafe but the only way this is working
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub fn new(
        descriptor: String,
        network: String,
        db_path: String,
        lnd_client: LightningClient,
    ) -> anyhow::Result<Self> {
        let parsed_network = network.parse::<Network>()?;
        // Set up bdk
        let wallet = Wallet::new(
            &descriptor,
            None,
            parsed_network,
            SqliteDatabase::new(db_path),
        )?;

        Ok(AppState { wallet, lnd_client })
    }
}

pub type SharedState = Arc<RwLock<AppState>>;
