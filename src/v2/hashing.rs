use mina_hasher::{Hashable, Hasher, ROInput};

use super::{generated, DataHashLibStateHashStableV1};
use super::state_hash::StateHashStable;

impl generated::MinaBaseStagedLedgerHashNonSnarkStableV1 {
    pub fn sha256(&self) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        let ledger_hash_bytes: Vec<u8> = self.ledger_hash.clone().into_inner().0.iter_bytes().rev().collect();
        
        hasher.update(&ledger_hash_bytes);
        hasher.update(self.aux_hash.clone().into_inner().0.as_ref());
        hasher.update(&self.pending_coinbase_aux.clone().into_inner().0.as_ref());
        hasher.finalize().to_vec()
    }
}
