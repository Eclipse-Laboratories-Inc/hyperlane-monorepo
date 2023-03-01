use async_trait::async_trait;

use hyperlane_core::{
    BlockInfo, ChainResult, HyperlaneChain, HyperlaneDomain, HyperlaneProvider, TxnInfo, H256,
};

/// A wrapper around a Sealevel provider to get generic blockchain information.
#[derive(Debug)]
pub struct SealevelProvider {
    domain: HyperlaneDomain,
}

impl HyperlaneChain for SealevelProvider {
    fn domain(&self) -> &HyperlaneDomain {
        &self.domain
    }
}

#[async_trait]
impl HyperlaneProvider for SealevelProvider {
    async fn get_block_by_hash(&self, _hash: &H256) -> ChainResult<BlockInfo> {
        todo!() // FIXME
    }

    async fn get_txn_by_hash(&self, _hash: &H256) -> ChainResult<TxnInfo> {
        todo!() // FIXME
    }
}
