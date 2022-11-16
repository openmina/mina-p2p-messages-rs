use mina_hasher::{Hashable, Hasher, ROInput};

use super::{generated, DataHashLibStateHashStableV1};
use super::state_hash::StateHashStable;
// use crate::versioned::{Ver, Versioned};

impl generated::MinaStateProtocolStateValueStableV2 {
    pub fn hash(&self, hasher: &mut impl Hasher<Self>) -> StateHashStable {
        let field = hasher.hash(self);
        DataHashLibStateHashStableV1(field.into()).into()
    }
}

impl Hashable for generated::UnsignedExtendedUInt32StableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        // TODO: should be u32 not i32?
        ROInput::new().append_u32(self.0.0 as u32)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::UnsignedExtendedUInt64StableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        // TODO: should be u64 not i64?
        ROInput::new().append_u64(self.0.0 as u64)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::MinaStateProtocolStateValueStableV2 {
    type D = ();

    fn to_roinput(&self) -> mina_hasher::ROInput {
        // TODO: hasher initialization is costly. Here we have to hash
        // `protocol_state.body` before we append it as a roinput, but
        // Hashable api isn't flexible enough to allow us to do that,
        // without initializing hasher here. This needs to be fixed/improved.
        let mut hasher = mina_hasher::create_kimchi(());
        let body_hash = hasher.hash(&self.body);
        ROInput::new()
            .append_hashable(&self.previous_state_hash)
            .append_field(body_hash)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("MinaProtoState".into())
    }
}

impl Hashable for generated::MinaStateProtocolStateBodyValueStableV2 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let roi = ROInput::new()
            .append_hashable(&self.constants)
            .append_hashable(&self.genesis_state_hash)
            .append_hashable(&self.blockchain_state)
            .append_hashable(&self.consensus_state);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        // None
        Some("MinaProtoStateBody".into())
    }
}

impl Hashable for generated::MinaBaseProtocolConstantsCheckedValueStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.k)
            .append_hashable(&self.delta)
            .append_hashable(&self.slots_per_epoch)
            .append_hashable(&self.slots_per_sub_window)
            .append_hashable(&self.genesis_state_timestamp)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::BlockTimeMakeStrTimeStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::MinaStateBlockchainStateValueStableV2 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.staged_ledger_hash)
            .append_hashable(&self.genesis_ledger_hash)
            .append_hashable(&self.registers)
            .append_hashable(&self.timestamp)
            .append_hashable(&self.body_reference)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::MinaBaseStagedLedgerHashStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.non_snark)
            .append_hashable(&self.pending_coinbase_hash.clone().into_inner().0.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

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

impl Hashable for generated::MinaBaseStagedLedgerHashNonSnarkStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_bytes(&self.sha256())
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::ConsensusProofOfStakeDataConsensusStateValueStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new()
            .append_hashable(&self.blockchain_length)
            .append_hashable(&self.epoch_count)
            .append_hashable(&self.min_window_density);
        for v in &self.sub_window_densities {
            roi = roi.append_hashable(v);
        }
        roi.append_hashable(&self.last_vrf_output)
            .append_hashable(&self.total_currency)
            .append_hashable(&self.curr_global_slot)
            .append_hashable(&self.global_slot_since_genesis)
            .append_bool(self.has_ancestor_in_same_checkpoint_window)
            .append_bool(self.supercharge_coinbase)
            .append_hashable(&self.staking_epoch_data)
            .append_hashable(&self.next_epoch_data)
            .append_hashable(&self.block_stake_winner.clone().into_inner().x)
            .append_bool(self.block_stake_winner.clone().into_inner().is_odd)
            .append_hashable(&self.block_creator.clone().into_inner().x)
            .append_bool(self.block_creator.clone().into_inner().is_odd)
            .append_hashable(&self.coinbase_receiver.clone().into_inner().x)
            .append_bool(self.coinbase_receiver.clone().into_inner().is_odd)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::ConsensusVrfOutputTruncatedStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let data = self.0.as_ref();
        let roi = ROInput::new();
        if data.len() <= 31 {
            roi.append_bytes(&data)
        } else {
            let roi = roi.append_bytes(&data[..31]);
            if data.len() > 31 {
                let last = data[31];
                roi.append_bool(last & 0b1 > 0)
                    .append_bool(last & 0b10 > 0)
                    .append_bool(last & 0b100 > 0)
                    .append_bool(last & 0b1000 > 0)
                    .append_bool(last & 0b10000 > 0)
            } else {
                roi
            }
        }
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::ConsensusGlobalSlotStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.slot_number)
            .append_hashable(&self.slots_per_epoch)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::ConsensusProofOfStakeDataEpochDataStakingValueVersionedValueStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.seed)
            .append_hashable(&self.start_checkpoint)
            .append_hashable(&self.epoch_length)
            .append_hashable(&self.ledger)
            .append_hashable(&self.lock_checkpoint)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for generated::MinaBaseEpochLedgerValueStableV1 {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.hash)
            .append_hashable(&self.total_currency)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::ConsensusProofOfStakeDataEpochDataNextValueVersionedValueStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.seed)
            .append_hashable(&self.start_checkpoint)
            .append_hashable(&self.epoch_length)
            .append_hashable(&self.ledger)
            .append_hashable(&self.lock_checkpoint)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::DataHashLibStateHashStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::MinaBaseEpochSeedStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::CurrencyMakeStrAmountMakeStrStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::MinaBaseLedgerHash0StableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::MinaStateBlockchainStateValueStableV2Registers
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.ledger)
            .append_hashable(&self.local_state)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::MinaTransactionLogicPartiesLogicLocalStateValueStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.stack_frame)
            .append_hashable(&self.call_stack)
            .append_hashable(&self.transaction_commitment)
            .append_hashable(&self.full_transaction_commitment)
            .append_hashable(&self.token_id)
            .append_hashable(&self.excess)
            .append_hashable(&self.ledger)
            .append_hashable(&self.account_update_index)
            .append_bool(self.success)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable
    for generated::MinaBaseStackFrameDigestStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

// MinaBaseCallStackDigestStableV1
impl Hashable
    for generated::MinaBaseCallStackDigestStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

// MinaBaseAccountIdMakeStrDigestStableV1
impl Hashable
    for generated::MinaBaseAccountIdMakeStrDigestStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_hashable(&self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

// MinaTransactionLogicPartiesLogicLocalStateValueStableV1Excess
impl Hashable
    for generated::MinaTransactionLogicPartiesLogicLocalStateValueStableV1Excess
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.magnitude)
            .append_bool(matches!(&self.sgn.0, generated::SgnStableV1::Pos))
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

// ConsensusBodyReferenceStableV1
impl Hashable
    for generated::ConsensusBodyReferenceStableV1
{
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_bytes(self.0.0.as_ref())
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}