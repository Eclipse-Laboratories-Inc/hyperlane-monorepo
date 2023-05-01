use hyperlane_core::{ChainCommunicationError, ChainResult};

use crate::solana::pubkey::Pubkey;

// FIXME is this really needed?
pub struct SealevelClient;
pub struct Provider;
impl Provider {
    fn new(_client: SealevelClient) -> Self {
        todo!() // FIXME
    }
}

fn pubkey_from_str<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: serde::Deserializer<'de>
{
    use std::str::FromStr as _;
    use serde::Deserialize as _;

    let pubkey = String::deserialize(deserializer)?;
    Pubkey::from_str(&pubkey)
        .map_err(serde::de::Error::custom)
}

/// Sealevel connection configuration
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ConnectionConf {
    /// Fully qualified string to connect to
    pub url: String,
    // TODO including this struct here seems like an unclean way to extend the config but just go
    // with it for now.
    pub sealevel: SealevelConfig,
}

/// Extra config needed for SVM chains that just doesn't fit the current config paradigm.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct SealevelConfig {
    pub mailbox_txn_payer_keypair: std::path::PathBuf,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub multisig_ism_validator_address: Pubkey, // TODO should be a vec
    #[serde(deserialize_with = "pubkey_from_str")]
    pub noop_cpi_log_program_id: Pubkey,
    pub token_bridge: Option<TokenBridgeConfig>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct TokenBridgeConfig {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub hyperlane_token_program_id: Pubkey,
    pub native_token_name: String,
    pub native_token_symbol: String,
    // TODO should be a vec of these
    pub erc20_token_name: String,
    pub erc20_token_symbol: String,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct SealevelNewConnectionError(#[from] anyhow::Error);

impl From<SealevelNewConnectionError> for ChainCommunicationError {
    fn from(err: SealevelNewConnectionError) -> Self {
        ChainCommunicationError::from_other(err)
    }
}

fn make_client(_conf: &ConnectionConf) -> ChainResult<SealevelClient> {
    // SealevelClient::new(&conf.url).map_err(|e| SealevelNewConnectionError(e).into())
    todo!() // FIXME
}

/// Create a new fuel provider and connection
pub fn make_provider(conf: &ConnectionConf) -> ChainResult<Provider> {
    Ok(Provider::new(make_client(conf)?))
}
