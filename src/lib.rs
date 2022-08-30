use binprot_derive::{BinProtRead, BinProtWrite};
use serde::{Deserialize, Serialize};

pub mod bigint;
pub mod char_;
#[cfg(feature = "hashing")]
mod hashing;
pub mod p2p;
pub mod phantom;
pub mod state_hash;
pub mod string;
pub mod versioned;

#[derive(Clone, Debug, Serialize, Deserialize, BinProtRead, BinProtWrite)]
#[serde(tag = "type", content = "message")]
pub enum GossipNetMessage {
    #[serde(rename = "external_transition")]
    NewState(p2p::MinaBlockExternalTransitionRawVersionedStable),
    #[serde(rename = "snark_pool_diff")]
    SnarkPoolDiff(p2p::NetworkPoolSnarkPoolDiffVersionedStable),
    #[serde(rename = "transaction_pool_diff")]
    TransactionPoolDiff(p2p::NetworkPoolTransactionPoolDiffVersionedStable),
}
