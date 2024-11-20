use crate::*;

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client;
use std::{env, path::PathBuf};

use dotenv::dotenv;

/// Configuration struct for the Relayer service.
///
/// This struct holds various configuration parameters needed for the Relayer service,
/// including SMTP settings, database path, web server address, and blockchain-related information.
#[derive(Clone)]
pub struct RelayerConfig {
    pub smtp_server: String,
    pub relayer_email_addr: String,
    pub db_path: String,
    pub web_server_address: String,
    pub regex_json_dir_path: PathBuf,
    pub prover_address: String,
    pub chain_rpc_provider: String,
    pub chain_rpc_explorer: String,
    pub chain_id: u32,
    pub private_key: String,
    pub email_account_recovery_version_id: u8,
    pub email_templates: String,
    pub error_email_addr: String,
    pub dkim_canister_id: String,
    pub wallet_canister_id: String,
    pub pem_path: String,
    pub ic_replica_url: String,
}

impl RelayerConfig {
    /// Creates a new instance of RelayerConfig.
    ///
    /// This function loads environment variables using dotenv and populates
    /// the RelayerConfig struct with the values.
    ///
    /// # Returns
    ///
    /// A new instance of RelayerConfig.
    pub async fn new() -> Self {
        // Load environment variables from .env file
        dotenv().ok();

        // Load environment variables from amazon secrets manager
        let aws_region = env::var("AWS_REGION").unwrap();
        let static_region: &'static str = Box::leak(aws_region.into_boxed_str());
        let region_provider =
            RegionProviderChain::default_provider().or_else(static_region);

        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&shared_config);


        let secret_id = env::var("AWS_SECRET_ACCESS_KEY").unwrap();
        let static_secret_id: &'static str = Box::leak(secret_id.into_boxed_str());
        let resp = client
            .get_secret_value()
            .secret_id(static_secret_id)
            .send()
            .await;

        println!(
            "Value: {}",
            resp.unwrap().secret_string().unwrap_or("No value!")
        );

        // Construct and return the RelayerConfig instance
        Self {
            smtp_server: env::var(SMTP_SERVER_KEY).unwrap(),
            relayer_email_addr: env::var(RELAYER_EMAIL_ADDR_KEY).unwrap(),
            db_path: env::var(DATABASE_PATH_KEY).unwrap(),
            web_server_address: env::var(WEB_SERVER_ADDRESS_KEY).unwrap(),
            regex_json_dir_path: env::var(REGEX_JSON_DIR_PATH_KEY).unwrap().into(),
            prover_address: env::var(PROVER_ADDRESS_KEY).unwrap(),
            chain_rpc_provider: env::var(CHAIN_RPC_PROVIDER_KEY).unwrap(),
            chain_rpc_explorer: env::var(CHAIN_RPC_EXPLORER_KEY).unwrap(),
            chain_id: env::var(CHAIN_ID_KEY).unwrap().parse().unwrap(),
            private_key: env::var(PRIVATE_KEY_KEY).unwrap(),
            email_account_recovery_version_id: env::var(EMAIL_ACCOUNT_RECOVERY_VERSION_ID_KEY)
                .unwrap()
                .parse()
                .unwrap(),
            email_templates: env::var(EMAIL_TEMPLATES_PATH_KEY).unwrap(),
            error_email_addr: env::var(ERROR_EMAIL_ADDR_KEY).unwrap(),
            dkim_canister_id: env::var(DKIM_CANISTER_ID_KEY).unwrap(),
            wallet_canister_id: env::var(WALLET_CANISTER_ID_KEY).unwrap(),
            pem_path: env::var(PEM_PATH_KEY).unwrap(),
            ic_replica_url: env::var(IC_REPLICA_URL_KEY).unwrap(),
        }
    }
}
