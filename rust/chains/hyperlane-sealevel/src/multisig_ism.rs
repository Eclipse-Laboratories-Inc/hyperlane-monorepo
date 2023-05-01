use async_trait::async_trait;
use hyperlane_core::{
    accumulator::merkle::Proof, ChainResult, ContractLocator, HyperlaneChain,
    HyperlaneContract, HyperlaneDomain, HyperlaneMessage, MultisigIsm, MultisigSignedCheckpoint,
    H256,
};
use tracing::warn;

use crate::{ConnectionConf, solana::pubkey::Pubkey};

/// A reference to a MultisigIsm contract on some Sealevel chain
#[derive(Debug)]
pub struct SealevelMultisigIsm {
    program_id: Pubkey,
    domain: HyperlaneDomain,
    validator_addresses: Vec<H256>
}

impl SealevelMultisigIsm {
    pub fn new(conf: &ConnectionConf, locator: ContractLocator) -> Self {
        // let rpc_client = RpcClient::new(conf.url.clone());
        let program_id = Pubkey::from(<[u8; 32]>::from(locator.address));
        let domain = locator.domain;
        let validator_addresses = vec![
            conf.sealevel.multisig_ism_validator_address.to_bytes().into(),
        ];

        Self {
            program_id,
            domain,
            validator_addresses,
        }
    }
}

impl HyperlaneContract for SealevelMultisigIsm {
    fn address(&self) -> H256 {
        self.program_id.to_bytes().into()
    }
}

impl HyperlaneChain for SealevelMultisigIsm {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }
}

#[async_trait]
impl MultisigIsm for SealevelMultisigIsm {
    /// Returns the validator and threshold needed to verify message
    async fn validators_and_threshold(
        &self,
        _message: &HyperlaneMessage,
    ) -> ChainResult<(Vec<H256>, u8)> {
        Ok((self.validator_addresses.clone(), self.validator_addresses.len().try_into().unwrap()))
    }

    /// Returns the metadata needed by the contract's verify function
    fn format_metadata(
        &self,
        _validators: &[H256],
        _threshold: u8,
        _checkpoint: &MultisigSignedCheckpoint,
        _proof: &Proof,
    ) -> Vec<u8> {
        // FIXME do something real
        warn!("Providing no formatted metadata for multisig ism");
        vec![]
    }
}
