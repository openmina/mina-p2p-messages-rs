/// Naive implementation for big integer.
pub type BigInt = [u8; 32];

/// Origin: Mina_block__Block.Stable.V2.t
/// Location: src/lib/mina_block/block.ml:8:4
pub struct MinaBlockBlockStableV2 {
    pub header: MinaBlockHeaderStableV2,
    pub body: StagedLedgerDiffBodyStableV1,
}

/// Origin: Network_pool__Transaction_pool.Diff_versioned.Stable.V2.t
/// Location: src/lib/network_pool/transaction_pool.ml:48:6
/// Location: src/std_internal.ml:131:2
/// Location: src/list0.ml:6:0
pub type NetworkPoolTransactionPoolDiffVersionedStableV2 = Vec<MinaBaseUserCommandStableV2>;

/// Origin: Network_pool__Snark_pool.Diff_versioned.Stable.V2.t
/// Location: src/lib/network_pool/snark_pool.ml:707:6
pub enum NetworkPoolSnarkPoolDiffVersionedStableV2 {
    AddSolvedWork(
        TransactionSnarkWorkStatementStableV2,
        NetworkPoolSnarkPoolDiffVersionedStableV2AddSolvedWork1,
    ),
    Empty,
}


/// Origin: Data_hash_lib__State_hash.Stable.V1.t
/// Location: src/lib/data_hash_lib/state_hash.ml:42:4
pub type DataHashLibStateHashStableV1 = BigInt;

/// Origin: Mina_base__Ledger_hash0.Stable.V1.t
/// Location: src/lib/mina_base/ledger_hash0.ml:17:4
pub type MinaBaseLedgerHash0StableV1 = BigInt;

/// Origin: Mina_base__Staged_ledger_hash.Aux_hash.Stable.V1.t
/// Location: src/lib/mina_base/staged_ledger_hash.ml:16:6
/// Location: src/std_internal.ml:140:2
/// Location: src/string.ml:44:6
pub type MinaBaseStagedLedgerHashAuxHashStableV1 = String;

/// Origin: Mina_base__Staged_ledger_hash.Pending_coinbase_aux.Stable.V1.t
/// Location: src/lib/mina_base/staged_ledger_hash.ml:60:6
/// Location: src/std_internal.ml:140:2
/// Location: src/string.ml:44:6
pub type MinaBaseStagedLedgerHashPendingCoinbaseAuxStableV1 = String;

/// Origin: Mina_base__Staged_ledger_hash.Non_snark.Stable.V1.t
/// Location: src/lib/mina_base/staged_ledger_hash.ml:96:6
pub struct MinaBaseStagedLedgerHashNonSnarkStableV1 {
    pub ledger_hash: MinaBaseLedgerHash0StableV1,
    pub aux_hash: MinaBaseStagedLedgerHashAuxHashStableV1,
    pub pending_coinbase_aux: MinaBaseStagedLedgerHashPendingCoinbaseAuxStableV1,
}

/// Origin: Mina_base__Pending_coinbase.Hash_builder.Stable.V1.t
/// Location: src/lib/mina_base/pending_coinbase.ml:356:6
pub type MinaBasePendingCoinbaseHashBuilderStableV1 = BigInt;

/// Origin: Mina_base__Pending_coinbase.Hash_versioned.Stable.V1.t
/// Location: src/lib/mina_base/pending_coinbase.ml:515:6
pub type MinaBasePendingCoinbaseHashVersionedStableV1 = MinaBasePendingCoinbaseHashBuilderStableV1;

/// Origin: Mina_base__Staged_ledger_hash.Stable.V1.t
/// Location: src/lib/mina_base/staged_ledger_hash.ml:200:4
/// Location: src/lib/mina_base/staged_ledger_hash.ml:183:6
pub struct MinaBaseStagedLedgerHashStableV1 {
    pub non_snark: MinaBaseStagedLedgerHashNonSnarkStableV1,
    pub pending_coinbase_hash: MinaBasePendingCoinbaseHashVersionedStableV1,
}

/// Origin: Mina_base__Stack_frame.Digest.Stable.V1.t
/// Location: src/lib/mina_base/stack_frame.ml:55:6
pub type MinaBaseStackFrameDigestStableV1 = BigInt;

/// Origin: Mina_base__Call_stack_digest.Stable.V1.t
/// Location: src/lib/mina_base/call_stack_digest.ml:6:4
pub type MinaBaseCallStackDigestStableV1 = BigInt;

/// Origin: Mina_base__Account_id.Digest.Stable.V1.t
/// Location: src/lib/mina_base/account_id.ml:53:6
pub type MinaBaseAccountIdDigestStableV1 = BigInt;

/// Origin: Unsigned_extended.UInt64.Stable.V1.t
/// Location: src/int64.ml:6:6
pub type UnsignedExtendedUInt64StableV1 = i64;

/// Origin: Currency.Amount.Make_str.Stable.V1.t
/// Location: src/lib/currency/currency.ml:992:8
pub type CurrencyAmountMakeStrStableV1 = UnsignedExtendedUInt64StableV1;

/// Origin: Sgn.Stable.V1.t
/// Location: src/lib/sgn/sgn.ml:9:4
pub enum SgnStableV1 {
    Pos,
    Neg,
}

/// Location: src/lib/currency/signed_poly.ml:6:4
pub struct MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg3 {
    pub magnitude: CurrencyAmountMakeStrStableV1,
    pub sgn: SgnStableV1,
}

/// Origin: Unsigned_extended.UInt32.Stable.V1.t
/// Location: src/int32.ml:6:6
pub type UnsignedExtendedUInt32StableV1 = i32;

/// Location: src/lib/mina_numbers/nat.ml:258:6
pub type MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg7 =
    UnsignedExtendedUInt32StableV1;

/// Origin: Mina_base__Transaction_status.Failure.Stable.V2.t
/// Location: src/lib/mina_base/transaction_status.ml:9:6
pub enum MinaBaseTransactionStatusFailureStableV2 {
    Predicate,
    SourceNotPresent,
    ReceiverNotPresent,
    AmountInsufficientToCreateAccount,
    CannotPayCreationFeeInToken,
    SourceInsufficientBalance,
    SourceMinimumBalanceViolation,
    ReceiverAlreadyExists,
    TokenOwnerNotCaller,
    Overflow,
    GlobalExcessOverflow,
    LocalExcessOverflow,
    SignedCommandOnZkappAccount,
    ZkappAccountNotPresent,
    UpdateNotPermittedBalance,
    UpdateNotPermittedTimingExistingAccount,
    UpdateNotPermittedDelegate,
    UpdateNotPermittedAppState,
    UpdateNotPermittedVerificationKey,
    UpdateNotPermittedSequenceState,
    UpdateNotPermittedZkappUri,
    UpdateNotPermittedTokenSymbol,
    UpdateNotPermittedPermissions,
    UpdateNotPermittedNonce,
    UpdateNotPermittedVotingFor,
    PartiesReplayCheckFailed,
    FeePayerNonceMustIncrease,
    FeePayerMustBeSigned,
    AccountBalancePreconditionUnsatisfied,
    AccountNoncePreconditionUnsatisfied,
    AccountReceiptChainHashPreconditionUnsatisfied,
    AccountDelegatePreconditionUnsatisfied,
    AccountSequenceStatePreconditionUnsatisfied,
    AccountAppStatePreconditionUnsatisfied(i32),
    AccountProvedStatePreconditionUnsatisfied,
    AccountIsNewPreconditionUnsatisfied,
    ProtocolStatePreconditionUnsatisfied,
    IncorrectNonce,
    InvalidFeeExcess,
}

/// Origin: Mina_base__Transaction_status.Failure.Collection.Stable.V1.t
/// Location: src/lib/mina_base/transaction_status.ml:71:8
/// Location: src/std_internal.ml:131:2
/// Location: src/list0.ml:6:0
pub type MinaBaseTransactionStatusFailureCollectionStableV1 =
    Vec<Vec<MinaBaseTransactionStatusFailureStableV2>>;

/// Origin: Mina_transaction_logic__Parties_logic.Local_state.Value.Stable.V1.t
/// Location: src/lib/transaction_logic/parties_logic.ml:216:8
/// Location: src/lib/transaction_logic/parties_logic.ml:170:6
pub struct MinaTransactionLogicPartiesLogicLocalStateValueStableV1 {
    pub stack_frame: MinaBaseStackFrameDigestStableV1,
    pub call_stack: MinaBaseCallStackDigestStableV1,
    pub transaction_commitment: BigInt,
    pub full_transaction_commitment: BigInt,
    pub token_id: MinaBaseAccountIdDigestStableV1,
    pub excess: MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg3,
    pub ledger: MinaBaseLedgerHash0StableV1,
    pub success: bool,
    pub party_index: MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg7,
    pub failure_status_tbl: MinaBaseTransactionStatusFailureCollectionStableV1,
}

/// Origin: Block_time.Time.Stable.V1.t
/// Location: src/lib/block_time/block_time.ml:14:6
pub type BlockTimeTimeStableV1 = UnsignedExtendedUInt64StableV1;

/// Origin: Blake2.Make.Stable.V1.t
/// Location: src/binable0.ml:120:10
pub type Blake2MakeStableV1 = String;

/// Origin: Consensus__Body_reference.Stable.V1.t
/// Location: src/lib/consensus/body_reference.ml:17:4
pub type ConsensusBodyReferenceStableV1 = Blake2MakeStableV1;

/// Location: src/lib/mina_state/registers.ml:8:4
pub struct MinaStateBlockchainStateValueStableV2Registers {
    pub ledger: MinaBaseLedgerHash0StableV1,
    pub pending_coinbase_stack: (),
    pub local_state: MinaTransactionLogicPartiesLogicLocalStateValueStableV1,
}

/// Origin: Mina_state__Blockchain_state.Value.Stable.V2.t
/// Location: src/lib/mina_state/blockchain_state.ml:43:6
/// Location: src/lib/mina_state/blockchain_state.ml:9:6
pub struct MinaStateBlockchainStateValueStableV2 {
    pub staged_ledger_hash: MinaBaseStagedLedgerHashStableV1,
    pub genesis_ledger_hash: MinaBaseLedgerHash0StableV1,
    pub registers: MinaStateBlockchainStateValueStableV2Registers,
    pub timestamp: BlockTimeTimeStableV1,
    pub body_reference: ConsensusBodyReferenceStableV1,
}

/// Location: src/lib/mina_numbers/nat.ml:258:6
pub type ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0 = UnsignedExtendedUInt32StableV1;

/// Origin: Consensus_vrf.Output.Truncated.Stable.V1.t
/// Location: src/lib/consensus/vrf/consensus_vrf.ml:167:8
/// Location: src/std_internal.ml:140:2
/// Location: src/string.ml:44:6
pub type ConsensusVrfOutputTruncatedStableV1 = String;

/// Location: src/lib/mina_numbers/nat.ml:258:6
pub type ConsensusGlobalSlotStableV1Arg0 = UnsignedExtendedUInt32StableV1;

/// Origin: Consensus__Global_slot.Stable.V1.t
/// Location: src/lib/consensus/global_slot.ml:21:4
/// Location: src/lib/consensus/global_slot.ml:11:6
pub struct ConsensusGlobalSlotStableV1 {
    pub slot_number: ConsensusGlobalSlotStableV1Arg0,
    pub slots_per_epoch: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
}

/// Origin: Mina_base__Epoch_ledger.Value.Stable.V1.t
/// Location: src/lib/mina_base/epoch_ledger.ml:20:6
/// Location: src/lib/mina_base/epoch_ledger.ml:9:6
pub struct MinaBaseEpochLedgerValueStableV1 {
    pub hash: MinaBaseLedgerHash0StableV1,
    pub total_currency: CurrencyAmountMakeStrStableV1,
}

/// Origin: Mina_base__Epoch_seed.Stable.V1.t
/// Location: src/lib/mina_base/epoch_seed.ml:18:4
pub type MinaBaseEpochSeedStableV1 = BigInt;

/// Origin: Consensus__Proof_of_stake.Data.Epoch_data.Staking_value_versioned.Value.Stable.V1.t
/// Location: src/lib/consensus/proof_of_stake.ml:1040:12
/// Location: src/lib/mina_base/epoch_data.ml:8:6
pub struct ConsensusProofOfStakeDataEpochDataStakingValueVersionedValueStableV1 {
    pub ledger: MinaBaseEpochLedgerValueStableV1,
    pub seed: MinaBaseEpochSeedStableV1,
    pub start_checkpoint: DataHashLibStateHashStableV1,
    pub lock_checkpoint: DataHashLibStateHashStableV1,
    pub epoch_length: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
}

/// Origin: Consensus__Proof_of_stake.Data.Epoch_data.Next_value_versioned.Value.Stable.V1.t
/// Location: src/lib/consensus/proof_of_stake.ml:1064:12
/// Location: src/lib/mina_base/epoch_data.ml:8:6
pub struct ConsensusProofOfStakeDataEpochDataNextValueVersionedValueStableV1 {
    pub ledger: MinaBaseEpochLedgerValueStableV1,
    pub seed: MinaBaseEpochSeedStableV1,
    pub start_checkpoint: DataHashLibStateHashStableV1,
    pub lock_checkpoint: DataHashLibStateHashStableV1,
    pub epoch_length: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
}

/// Origin: Non_zero_curve_point.Uncompressed.Stable.V1.t
/// Location: src/lib/non_zero_curve_point/non_zero_curve_point.ml:44:6
/// Location: src/lib/non_zero_curve_point/compressed_poly.ml:13:6
pub struct NonZeroCurvePointUncompressedStableV1 {
    pub x: BigInt,
    pub is_odd: bool,
}

/// Origin: Consensus__Proof_of_stake.Data.Consensus_state.Value.Stable.V1.t
/// Location: src/lib/consensus/proof_of_stake.ml:1708:10
/// Location: src/lib/consensus/proof_of_stake.ml:1673:10
pub struct ConsensusProofOfStakeDataConsensusStateValueStableV1 {
    pub blockchain_length: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub epoch_count: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub min_window_density: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub sub_window_densities: Vec<ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0>,
    pub last_vrf_output: ConsensusVrfOutputTruncatedStableV1,
    pub total_currency: CurrencyAmountMakeStrStableV1,
    pub curr_global_slot: ConsensusGlobalSlotStableV1,
    pub global_slot_since_genesis: ConsensusGlobalSlotStableV1Arg0,
    pub staking_epoch_data: ConsensusProofOfStakeDataEpochDataStakingValueVersionedValueStableV1,
    pub next_epoch_data: ConsensusProofOfStakeDataEpochDataNextValueVersionedValueStableV1,
    pub has_ancestor_in_same_checkpoint_window: bool,
    pub block_stake_winner: NonZeroCurvePointUncompressedStableV1,
    pub block_creator: NonZeroCurvePointUncompressedStableV1,
    pub coinbase_receiver: NonZeroCurvePointUncompressedStableV1,
    pub supercharge_coinbase: bool,
}

/// Origin: Mina_base__Protocol_constants_checked.Value.Stable.V1.t
/// Location: src/lib/mina_base/protocol_constants_checked.ml:22:6
/// Location: src/lib/genesis_constants/genesis_constants.ml:239:8
pub struct MinaBaseProtocolConstantsCheckedValueStableV1 {
    pub k: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub slots_per_epoch: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub slots_per_sub_window: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub delta: ConsensusProofOfStakeDataConsensusStateValueStableV1Arg0,
    pub genesis_state_timestamp: BlockTimeTimeStableV1,
}

/// Origin: Mina_state__Protocol_state.Body.Value.Stable.V2.t
/// Location: src/lib/mina_state/protocol_state.ml:53:8
/// Location: src/lib/mina_state/protocol_state.ml:38:8
pub struct MinaStateProtocolStateBodyValueStableV2 {
    pub genesis_state_hash: DataHashLibStateHashStableV1,
    pub blockchain_state: MinaStateBlockchainStateValueStableV2,
    pub consensus_state: ConsensusProofOfStakeDataConsensusStateValueStableV1,
    pub constants: MinaBaseProtocolConstantsCheckedValueStableV1,
}

/// Origin: Mina_state__Protocol_state.Value.Stable.V2.t
/// Location: src/lib/mina_state/protocol_state.ml:177:6
/// Location: src/lib/mina_state/protocol_state.ml:16:6
pub struct MinaStateProtocolStateValueStableV2 {
    pub previous_state_hash: DataHashLibStateHashStableV1,
    pub body: MinaStateProtocolStateBodyValueStableV2,
}

/// Origin: Limb_vector__Constant.Hex64.Stable.V1.t
/// Location: src/lib/pickles/limb_vector/constant.ml:60:6
pub type LimbVectorConstantHex64StableV1 = UnsignedExtendedUInt64StableV1;

/// Location: src/lib/crypto/kimchi_backend/common/scalar_challenge.ml:6:4
pub struct PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0Arg0 {
    pub inner: (
        LimbVectorConstantHex64StableV1,
        (LimbVectorConstantHex64StableV1, ()),
    ),
}

/// Location: src/lib/pickles/composition_types/bulletproof_challenge.ml:6:4
pub struct PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 {
    pub prechallenge: PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0Arg0,
}

/// Origin: Pickles__Reduced_me_only.Wrap.Challenges_vector.Stable.V2.t
/// Location: src/lib/pickles/reduced_me_only.ml:45:8
/// Location: src/lib/crypto/kimchi_backend/pasta/basic.ml:32:8
pub type PicklesReducedMeOnlyWrapChallengesVectorStableV2 = (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , ())))))))))))))) ,) ;

/// Location: src/lib/pickles/composition_types/composition_types.ml:268:10
pub struct PicklesProofProofsVerified2ReprStableV2Arg0 {
    pub challenge_polynomial_commitment: (BigInt, BigInt),
    pub old_bulletproof_challenges: (
        PicklesReducedMeOnlyWrapChallengesVectorStableV2,
        (PicklesReducedMeOnlyWrapChallengesVectorStableV2, ()),
    ),
}

/// Location: src/lib/pickles/reduced_me_only.ml:16:6
pub struct PicklesProofProofsVerified2ReprStableV2Arg1 { pub app_state : () , pub challenge_polynomial_commitments : Vec < (BigInt , BigInt) > , pub old_bulletproof_challenges : Vec < (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , ())))))))))))))))) > , }

/// Location: src/lib/pickles_types/shifted_value.ml:94:6
pub enum PicklesProofProofsVerified2ReprStableV2StatementArg2 {
    ShiftedValue(BigInt),
}

/// Origin: Composition_types__Digest.Constant.Stable.V1.t
/// Location: src/lib/pickles/composition_types/digest.ml:13:6
pub type CompositionTypesDigestConstantStableV1 = (
    LimbVectorConstantHex64StableV1,
    (
        LimbVectorConstantHex64StableV1,
        (
            LimbVectorConstantHex64StableV1,
            (LimbVectorConstantHex64StableV1, ()),
        ),
    ),
);

/// Origin: Pickles_base__Proofs_verified.Stable.V1.t
/// Location: src/lib/pickles_base/proofs_verified.ml:7:4
pub enum PicklesBaseProofsVerifiedStableV1 {
    N0,
    N1,
    N2,
}

/// Origin: Composition_types__Branch_data.Domain_log2.Stable.V1.t
/// Location: src/lib/pickles/composition_types/branch_data.ml:13:6
/// Location: src/std_internal.ml:113:2
/// Location: src/char.ml:8:6
pub type CompositionTypesBranchDataDomainLog2StableV1 = char;

/// Origin: Composition_types__Branch_data.Stable.V1.t
/// Location: src/lib/pickles/composition_types/branch_data.ml:40:4
pub struct CompositionTypesBranchDataStableV1 {
    pub proofs_verified: PicklesBaseProofsVerifiedStableV1,
    pub domain_log2: CompositionTypesBranchDataDomainLog2StableV1,
}

/// Location: src/lib/pickles/composition_types/composition_types.ml:35:14
pub struct PicklesProofProofsVerified2ReprStableV2StatementArg0 {
    pub alpha: PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0Arg0,
    pub beta: (
        LimbVectorConstantHex64StableV1,
        (LimbVectorConstantHex64StableV1, ()),
    ),
    pub gamma: (
        LimbVectorConstantHex64StableV1,
        (LimbVectorConstantHex64StableV1, ()),
    ),
    pub zeta: PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0Arg0,
}

/// Location: src/lib/pickles/composition_types/composition_types.ml:139:10
pub struct PicklesProofProofsVerified2ReprStableV2StatementProofStateDeferredValues { pub plonk : PicklesProofProofsVerified2ReprStableV2StatementArg0 , pub combined_inner_product : PicklesProofProofsVerified2ReprStableV2StatementArg2 , pub b : PicklesProofProofsVerified2ReprStableV2StatementArg2 , pub xi : PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0Arg0 , pub bulletproof_challenges : (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , (PicklesReducedMeOnlyWrapChallengesVectorStableV2Arg0 , ())))))))))))))))) , pub branch_data : CompositionTypesBranchDataStableV1 , }

/// Location: src/lib/pickles/composition_types/composition_types.ml:295:8
pub struct PicklesProofProofsVerified2ReprStableV2StatementProofState {
    pub deferred_values: PicklesProofProofsVerified2ReprStableV2StatementProofStateDeferredValues,
    pub sponge_digest_before_evaluations: CompositionTypesDigestConstantStableV1,
    pub me_only: PicklesProofProofsVerified2ReprStableV2Arg0,
}

/// Location: src/lib/pickles/composition_types/composition_types.ml:506:10
/// Location: src/lib/pickles/composition_types/composition_types.ml:476:8
pub struct PicklesProofProofsVerified2ReprStableV2Statement {
    pub proof_state: PicklesProofProofsVerified2ReprStableV2StatementProofState,
    pub pass_through: PicklesProofProofsVerified2ReprStableV2Arg1,
}

/// Location: src/lib/pickles_types/plonk_types.ml:150:8
pub struct PicklesProofProofsVerified2ReprStableV2PrevEvalsEvalsEvalsLookupArg0 {
    pub sorted: Vec<(Vec<BigInt>, Vec<BigInt>)>,
    pub aggreg: (Vec<BigInt>, Vec<BigInt>),
    pub table: (Vec<BigInt>, Vec<BigInt>),
    pub runtime: Option<(Vec<BigInt>, Vec<BigInt>)>,
}

/// Location: src/lib/pickles_types/plonk_types.ml:226:6
pub struct PicklesProofProofsVerified2ReprStableV2PrevEvalsEvalsEvals {
    pub w: (
        (Vec<BigInt>, Vec<BigInt>),
        (
            (Vec<BigInt>, Vec<BigInt>),
            (
                (Vec<BigInt>, Vec<BigInt>),
                (
                    (Vec<BigInt>, Vec<BigInt>),
                    (
                        (Vec<BigInt>, Vec<BigInt>),
                        (
                            (Vec<BigInt>, Vec<BigInt>),
                            (
                                (Vec<BigInt>, Vec<BigInt>),
                                (
                                    (Vec<BigInt>, Vec<BigInt>),
                                    (
                                        (Vec<BigInt>, Vec<BigInt>),
                                        (
                                            (Vec<BigInt>, Vec<BigInt>),
                                            (
                                                (Vec<BigInt>, Vec<BigInt>),
                                                (
                                                    (Vec<BigInt>, Vec<BigInt>),
                                                    (
                                                        (Vec<BigInt>, Vec<BigInt>),
                                                        (
                                                            (Vec<BigInt>, Vec<BigInt>),
                                                            ((Vec<BigInt>, Vec<BigInt>), ()),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
    pub z: (Vec<BigInt>, Vec<BigInt>),
    pub s: (
        (Vec<BigInt>, Vec<BigInt>),
        (
            (Vec<BigInt>, Vec<BigInt>),
            (
                (Vec<BigInt>, Vec<BigInt>),
                (
                    (Vec<BigInt>, Vec<BigInt>),
                    ((Vec<BigInt>, Vec<BigInt>), ((Vec<BigInt>, Vec<BigInt>), ())),
                ),
            ),
        ),
    ),
    pub generic_selector: (Vec<BigInt>, Vec<BigInt>),
    pub poseidon_selector: (Vec<BigInt>, Vec<BigInt>),
    pub lookup: Option<PicklesProofProofsVerified2ReprStableV2PrevEvalsEvalsEvalsLookupArg0>,
}

/// Location: src/lib/pickles_types/plonk_types.ml:416:8
pub struct PicklesProofProofsVerified2ReprStableV2PrevEvalsEvals {
    pub public_input: (BigInt, BigInt),
    pub evals: PicklesProofProofsVerified2ReprStableV2PrevEvalsEvalsEvals,
}

/// Location: src/lib/pickles_types/plonk_types.ml:449:6
pub struct PicklesProofProofsVerified2ReprStableV2PrevEvals {
    pub evals: PicklesProofProofsVerified2ReprStableV2PrevEvalsEvals,
    pub ft_eval1: BigInt,
}

/// Location: src/lib/pickles_types/plonk_types.ml:599:8
pub struct PicklesProofProofsVerified2ReprStableV2ProofMessagesLookupArg0 {
    pub sorted: Vec<Vec<(BigInt, BigInt)>>,
    pub aggreg: Vec<(BigInt, BigInt)>,
    pub runtime: Option<Vec<(BigInt, BigInt)>>,
}

/// Location: src/lib/pickles_types/plonk_types.ml:656:6
pub struct PicklesProofProofsVerified2ReprStableV2ProofMessages {
    pub w_comm: (
        Vec<(BigInt, BigInt)>,
        (
            Vec<(BigInt, BigInt)>,
            (
                Vec<(BigInt, BigInt)>,
                (
                    Vec<(BigInt, BigInt)>,
                    (
                        Vec<(BigInt, BigInt)>,
                        (
                            Vec<(BigInt, BigInt)>,
                            (
                                Vec<(BigInt, BigInt)>,
                                (
                                    Vec<(BigInt, BigInt)>,
                                    (
                                        Vec<(BigInt, BigInt)>,
                                        (
                                            Vec<(BigInt, BigInt)>,
                                            (
                                                Vec<(BigInt, BigInt)>,
                                                (
                                                    Vec<(BigInt, BigInt)>,
                                                    (
                                                        Vec<(BigInt, BigInt)>,
                                                        (
                                                            Vec<(BigInt, BigInt)>,
                                                            (Vec<(BigInt, BigInt)>, ()),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
    pub z_comm: Vec<(BigInt, BigInt)>,
    pub t_comm: Vec<(BigInt, BigInt)>,
    pub lookup: Option<PicklesProofProofsVerified2ReprStableV2ProofMessagesLookupArg0>,
}

/// Location: src/lib/pickles_types/plonk_types.ml:496:8
pub struct PicklesProofProofsVerified2ReprStableV2ProofOpeningsProof {
    pub lr: Vec<((BigInt, BigInt), (BigInt, BigInt))>,
    pub z_1: BigInt,
    pub z_2: BigInt,
    pub delta: (BigInt, BigInt),
    pub challenge_polynomial_commitment: (BigInt, BigInt),
}

/// Location: src/lib/pickles_types/plonk_types.ml:518:6
pub struct PicklesProofProofsVerified2ReprStableV2ProofOpenings {
    pub proof: PicklesProofProofsVerified2ReprStableV2ProofOpeningsProof,
    pub evals: PicklesProofProofsVerified2ReprStableV2PrevEvalsEvalsEvals,
    pub ft_eval1: BigInt,
}

/// Location: src/lib/crypto/kimchi_backend/common/plonk_dlog_proof.ml:160:6
/// Location: src/lib/pickles_types/plonk_types.ml:707:6
pub struct PicklesProofProofsVerified2ReprStableV2Proof {
    pub messages: PicklesProofProofsVerified2ReprStableV2ProofMessages,
    pub openings: PicklesProofProofsVerified2ReprStableV2ProofOpenings,
}

/// Origin: Pickles__Proof.Proofs_verified_2.Repr.Stable.V2.t
/// Location: src/lib/pickles/proof.ml:326:8
/// Location: src/lib/pickles/proof.ml:46:8
pub struct PicklesProofProofsVerified2ReprStableV2 {
    pub statement: PicklesProofProofsVerified2ReprStableV2Statement,
    pub prev_evals: PicklesProofProofsVerified2ReprStableV2PrevEvals,
    pub proof: PicklesProofProofsVerified2ReprStableV2Proof,
}

/// Origin: Mina_base__Proof.Stable.V2.t
/// Location: src/lib/mina_base/proof.ml:12:4
pub type MinaBaseProofStableV2 = PicklesProofProofsVerified2ReprStableV2;

/// Origin: Mina_base__State_body_hash.Stable.V1.t
/// Location: src/lib/mina_base/state_body_hash.ml:19:4
pub type MinaBaseStateBodyHashStableV1 = BigInt;

/// Origin: Protocol_version.Stable.V1.t
/// Location: src/lib/protocol_version/protocol_version.ml:8:4
pub struct ProtocolVersionStableV1 {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

/// Origin: Mina_block__Header.Stable.V2.t
/// Location: src/lib/mina_block/header.ml:14:4
pub struct MinaBlockHeaderStableV2 {
    pub protocol_state: MinaStateProtocolStateValueStableV2,
    pub protocol_state_proof: MinaBaseProofStableV2,
    pub delta_block_chain_proof: (
        DataHashLibStateHashStableV1,
        Vec<MinaBaseStateBodyHashStableV1>,
    ),
    pub current_protocol_version: ProtocolVersionStableV1,
    pub proposed_protocol_version_opt: Option<ProtocolVersionStableV1>,
}

/// Origin: Currency.Fee.Stable.V1.t
/// Location: src/lib/currency/currency.ml:862:6
pub type CurrencyFeeStableV1 = UnsignedExtendedUInt64StableV1;

/// Origin: Mina_base__Pending_coinbase.Coinbase_stack.Stable.V1.t
/// Location: src/lib/mina_base/pending_coinbase.ml:150:6
pub type MinaBasePendingCoinbaseCoinbaseStackStableV1 = BigInt;

/// Origin: Mina_base__Pending_coinbase.Stack_hash.Stable.V1.t
/// Location: src/lib/mina_base/pending_coinbase.ml:210:6
pub type MinaBasePendingCoinbaseStackHashStableV1 = BigInt;

/// Origin: Mina_base__Pending_coinbase.State_stack.Stable.V1.t
/// Location: src/lib/mina_base/pending_coinbase.ml:245:6
/// Location: src/lib/mina_base/pending_coinbase.ml:236:8
pub struct MinaBasePendingCoinbaseStateStackStableV1 {
    pub init: MinaBasePendingCoinbaseStackHashStableV1,
    pub curr: MinaBasePendingCoinbaseStackHashStableV1,
}

/// Origin: Mina_base__Pending_coinbase.Stack_versioned.Stable.V1.t
/// Location: src/lib/mina_base/pending_coinbase.ml:502:6
/// Location: src/lib/mina_base/pending_coinbase.ml:492:8
pub struct MinaBasePendingCoinbaseStackVersionedStableV1 {
    pub data: MinaBasePendingCoinbaseCoinbaseStackStableV1,
    pub state: MinaBasePendingCoinbaseStateStackStableV1,
}

/// Origin: Mina_base__Fee_excess.Stable.V1.t
/// Location: src/lib/mina_base/fee_excess.ml:123:4
/// Location: src/lib/mina_base/fee_excess.ml:54:6
pub struct MinaBaseFeeExcessStableV1 {
    pub fee_token_l: MinaBaseAccountIdDigestStableV1,
    pub fee_excess_l: MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg3,
    pub fee_token_r: MinaBaseAccountIdDigestStableV1,
    pub fee_excess_r: MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg3,
}

/// Origin: Mina_base__Account.Token_symbol.Stable.V1.t
/// Location: src/string.ml:14:6
pub type MinaBaseAccountTokenSymbolStableV1 = String;

/// Origin: Transaction_snark.Statement.With_sok.Stable.V2.t
/// Location: src/lib/transaction_snark/transaction_snark.ml:223:8
/// Location: src/lib/transaction_snark/transaction_snark.ml:122:8
pub struct TransactionSnarkStatementWithSokStableV2 {
    pub source: MinaStateBlockchainStateValueStableV2Registers,
    pub target: MinaStateBlockchainStateValueStableV2Registers,
    pub supply_increase: CurrencyAmountMakeStrStableV1,
    pub fee_excess: MinaBaseFeeExcessStableV1,
    pub sok_digest: MinaBaseAccountTokenSymbolStableV1,
}

/// Origin: Transaction_snark.Proof.Stable.V2.t
/// Location: src/lib/transaction_snark/transaction_snark.ml:375:6
pub type TransactionSnarkProofStableV2 = PicklesProofProofsVerified2ReprStableV2;

/// Origin: Transaction_snark.Stable.V2.t
/// Location: src/lib/transaction_snark/transaction_snark.ml:386:4
pub struct TransactionSnarkStableV2 {
    pub statement: TransactionSnarkStatementWithSokStableV2,
    pub proof: TransactionSnarkProofStableV2,
}

/// Origin: Ledger_proof.Prod.Stable.V2.t
/// Location: src/lib/ledger_proof/ledger_proof.ml:10:6
pub type LedgerProofProdStableV2 = TransactionSnarkStableV2;

/// Location: src/lib/one_or_two/one_or_two.ml:7:4
pub enum TransactionSnarkWorkTStableV2Proofs {
    One(LedgerProofProdStableV2),
    Two((LedgerProofProdStableV2, LedgerProofProdStableV2)),
}

/// Origin: Transaction_snark_work.T.Stable.V2.t
/// Location: src/lib/transaction_snark_work/transaction_snark_work.ml:82:6
pub struct TransactionSnarkWorkTStableV2 {
    pub fee: CurrencyFeeStableV1,
    pub proofs: TransactionSnarkWorkTStableV2Proofs,
    pub prover: NonZeroCurvePointUncompressedStableV1,
}

/// Location: src/lib/mina_numbers/nat.ml:258:6
pub type MinaBaseSignedCommandPayloadCommonStableV2Arg2 = UnsignedExtendedUInt32StableV1;

/// Origin: Mina_base__Signed_command_memo.Stable.V1.t
/// Location: src/lib/mina_base/signed_command_memo.ml:11:4
/// Location: src/std_internal.ml:140:2
/// Location: src/string.ml:44:6
pub type MinaBaseSignedCommandMemoStableV1 = String;

/// Origin: Mina_base__Signed_command_payload.Common.Stable.V2.t
/// Location: src/lib/mina_base/signed_command_payload.ml:67:6
/// Location: src/lib/mina_base/signed_command_payload.ml:40:8
pub struct MinaBaseSignedCommandPayloadCommonStableV2 {
    pub fee: CurrencyFeeStableV1,
    pub fee_payer_pk: NonZeroCurvePointUncompressedStableV1,
    pub nonce: MinaBaseSignedCommandPayloadCommonStableV2Arg2,
    pub valid_until: ConsensusGlobalSlotStableV1Arg0,
    pub memo: MinaBaseSignedCommandMemoStableV1,
}

/// Origin: Mina_base__Payment_payload.Stable.V2.t
/// Location: src/lib/mina_base/payment_payload.ml:24:4
/// Location: src/lib/mina_base/payment_payload.ml:14:6
pub struct MinaBasePaymentPayloadStableV2 {
    pub source_pk: NonZeroCurvePointUncompressedStableV1,
    pub receiver_pk: NonZeroCurvePointUncompressedStableV1,
    pub amount: CurrencyAmountMakeStrStableV1,
}

/// Origin: Mina_base__Stake_delegation.Stable.V1.t
/// Location: src/lib/mina_base/stake_delegation.ml:9:4
pub enum MinaBaseStakeDelegationStableV1 {
    SetDelegate {
        delegator: NonZeroCurvePointUncompressedStableV1,
        new_delegate: NonZeroCurvePointUncompressedStableV1,
    },
}

/// Origin: Mina_base__Signed_command_payload.Body.Stable.V2.t
/// Location: src/lib/mina_base/signed_command_payload.ml:177:6
pub enum MinaBaseSignedCommandPayloadBodyStableV2 {
    Payment(MinaBasePaymentPayloadStableV2),
    StakeDelegation(MinaBaseStakeDelegationStableV1),
}

/// Origin: Mina_base__Signed_command_payload.Stable.V2.t
/// Location: src/lib/mina_base/signed_command_payload.ml:258:4
/// Location: src/lib/mina_base/signed_command_payload.ml:244:6
pub struct MinaBaseSignedCommandPayloadStableV2 {
    pub common: MinaBaseSignedCommandPayloadCommonStableV2,
    pub body: MinaBaseSignedCommandPayloadBodyStableV2,
}

/// Origin: Mina_base__Signature.Stable.V1.t
/// Location: src/lib/mina_base/signature.ml:18:4
/// Location: src/lib/mina_base/signature_poly.ml:6:4
pub type MinaBaseSignatureStableV1 = (BigInt, BigInt);

/// Origin: Mina_base__Signed_command.Stable.V2.t
/// Location: src/lib/mina_base/signed_command.ml:23:4
/// Location: src/lib/mina_base/signed_command.ml:13:6
pub struct MinaBaseSignedCommandStableV2 {
    pub payload: MinaBaseSignedCommandPayloadStableV2,
    pub signer: NonZeroCurvePointUncompressedStableV1,
    pub signature: MinaBaseSignatureStableV1,
}

/// Origin: Mina_base__Party.Body.Fee_payer.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:958:8
pub struct MinaBasePartyBodyFeePayerStableV1 {
    pub public_key: NonZeroCurvePointUncompressedStableV1,
    pub fee: CurrencyFeeStableV1,
    pub valid_until: Option<ConsensusGlobalSlotStableV1Arg0>,
    pub nonce: MinaBaseSignedCommandPayloadCommonStableV2Arg2,
}

/// Origin: Mina_base__Party.Fee_payer.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:1349:6
pub struct MinaBasePartyFeePayerStableV1 {
    pub body: MinaBasePartyBodyFeePayerStableV1,
    pub authorization: MinaBaseSignatureStableV1,
}

/// Location: src/lib/mina_base/zkapp_basic.ml:100:6
pub enum MinaBasePartyUpdateStableV1AppStateArg0 {
    Set(BigInt),
    Keep,
}

/// Origin: Mina_base__Party.Update.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:219:6
pub struct MinaBasePartyUpdateStableV1 {
    pub app_state: (
        MinaBasePartyUpdateStableV1AppStateArg0,
        (
            MinaBasePartyUpdateStableV1AppStateArg0,
            (
                MinaBasePartyUpdateStableV1AppStateArg0,
                (
                    MinaBasePartyUpdateStableV1AppStateArg0,
                    (
                        MinaBasePartyUpdateStableV1AppStateArg0,
                        (
                            MinaBasePartyUpdateStableV1AppStateArg0,
                            (
                                MinaBasePartyUpdateStableV1AppStateArg0,
                                (MinaBasePartyUpdateStableV1AppStateArg0, ()),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
    pub delegate: MinaBasePartyUpdateStableV1AppStateArg0,
    pub verification_key: MinaBasePartyUpdateStableV1AppStateArg0,
    pub permissions: MinaBasePartyUpdateStableV1AppStateArg0,
    pub zkapp_uri: MinaBasePartyUpdateStableV1AppStateArg0,
    pub token_symbol: MinaBasePartyUpdateStableV1AppStateArg0,
    pub timing: MinaBasePartyUpdateStableV1AppStateArg0,
    pub voting_for: MinaBasePartyUpdateStableV1AppStateArg0,
}

/// Origin: Mina_base__Party.Body.Events'.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:724:8
/// Location: src/std_internal.ml:131:2
/// Location: src/list0.ml:6:0
pub type MinaBasePartyBodyEventsStableV1 = Vec<Vec<BigInt>>;

/// Location: src/lib/mina_base/zkapp_basic.ml:230:6
pub enum MinaBaseZkappPreconditionProtocolStateStableV1Arg0 {
    Check(MinaBaseLedgerHash0StableV1),
    Ignore,
}

/// Location: src/lib/mina_base/zkapp_precondition.ml:23:6
pub struct MinaBaseZkappPreconditionProtocolStateStableV1Arg1Arg0 {
    pub lower: BlockTimeTimeStableV1,
    pub upper: BlockTimeTimeStableV1,
}

/// Location: src/lib/mina_base/zkapp_precondition.ml:176:6
/// Location: src/lib/mina_base/zkapp_basic.ml:230:6
pub enum MinaBaseZkappPreconditionProtocolStateStableV1Arg1 {
    Check(MinaBaseZkappPreconditionProtocolStateStableV1Arg1Arg0),
    Ignore,
}

/// Origin: Mina_base__Zkapp_precondition.Protocol_state.Epoch_data.Stable.V1.t
/// Location: src/lib/mina_base/zkapp_precondition.ml:803:8
/// Location: src/lib/mina_base/epoch_data.ml:8:6
pub struct MinaBaseZkappPreconditionProtocolStateEpochDataStableV1 {
    pub ledger: MinaBaseEpochLedgerValueStableV1,
    pub seed: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub start_checkpoint: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub lock_checkpoint: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub epoch_length: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
}

/// Origin: Mina_base__Zkapp_precondition.Protocol_state.Stable.V1.t
/// Location: src/lib/mina_base/zkapp_precondition.ml:973:6
/// Location: src/lib/mina_base/zkapp_precondition.ml:934:8
pub struct MinaBaseZkappPreconditionProtocolStateStableV1 {
    pub snarked_ledger_hash: MinaBaseZkappPreconditionProtocolStateStableV1Arg0,
    pub timestamp: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub blockchain_length: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub min_window_density: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub last_vrf_output: (),
    pub total_currency: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub global_slot_since_hard_fork: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub global_slot_since_genesis: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub staking_epoch_data: MinaBaseZkappPreconditionProtocolStateEpochDataStableV1,
    pub next_epoch_data: MinaBaseZkappPreconditionProtocolStateEpochDataStableV1,
}

/// Origin: Mina_base__Zkapp_precondition.Account.Stable.V2.t
/// Location: src/lib/mina_base/zkapp_precondition.ml:483:6
pub struct MinaBaseZkappPreconditionAccountStableV2 {
    pub balance: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub nonce: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub receipt_chain_hash: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub delegate: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub state: (
        MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
        (
            MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
            (
                MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
                (
                    MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
                    (
                        MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
                        (
                            MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
                            (
                                MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
                                (MinaBaseZkappPreconditionProtocolStateStableV1Arg1, ()),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
    pub sequence_state: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub proved_state: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
    pub is_new: MinaBaseZkappPreconditionProtocolStateStableV1Arg1,
}

/// Origin: Mina_base__Party.Account_precondition.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:505:6
pub enum MinaBasePartyAccountPreconditionStableV1 {
    Full(MinaBaseZkappPreconditionAccountStableV2),
    Nonce(MinaBaseSignedCommandPayloadCommonStableV2Arg2),
    Accept,
}

/// Origin: Mina_base__Party.Preconditions.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:648:6
pub struct MinaBasePartyPreconditionsStableV1 {
    pub network: MinaBaseZkappPreconditionProtocolStateStableV1,
    pub account: MinaBasePartyAccountPreconditionStableV1,
}

/// Origin: Mina_base__Party.Call_type.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:27:6
pub enum MinaBasePartyCallTypeStableV1 {
    Call,
    DelegateCall,
}

/// Origin: Mina_base__Party.Body.Wire.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:736:8
pub struct MinaBasePartyBodyWireStableV1 {
    pub public_key: NonZeroCurvePointUncompressedStableV1,
    pub token_id: MinaBaseAccountIdDigestStableV1,
    pub update: MinaBasePartyUpdateStableV1,
    pub balance_change: MinaTransactionLogicPartiesLogicLocalStateValueStableV1Arg3,
    pub increment_nonce: bool,
    pub events: MinaBasePartyBodyEventsStableV1,
    pub sequence_events: MinaBasePartyBodyEventsStableV1,
    pub call_data: BigInt,
    pub preconditions: MinaBasePartyPreconditionsStableV1,
    pub use_full_commitment: bool,
    pub caller: MinaBasePartyCallTypeStableV1,
}

/// Origin: Pickles__Proof.Proofs_verified_max.Stable.V2.t
/// Location: src/lib/pickles/proof.ml:395:8
/// Location: src/lib/pickles/proof.ml:46:8
pub struct PicklesProofProofsVerifiedMaxStableV2 {
    pub statement: PicklesProofProofsVerified2ReprStableV2Statement,
    pub prev_evals: PicklesProofProofsVerified2ReprStableV2PrevEvals,
    pub proof: PicklesProofProofsVerified2ReprStableV2Proof,
}

/// Origin: Mina_base__Control.Stable.V2.t
/// Location: src/lib/mina_base/control.ml:11:4
pub enum MinaBaseControlStableV2 {
    Proof(PicklesProofProofsVerifiedMaxStableV2),
    Signature(MinaBaseSignatureStableV1),
    NoneGiven,
}

/// Origin: Mina_base__Party.T.Wire.Stable.V1.t
/// Location: src/lib/mina_base/party.ml:1276:8
pub struct MinaBasePartyTWireStableV1 {
    pub body: MinaBasePartyBodyWireStableV1,
    pub authorization: MinaBaseControlStableV2,
}

/// Location: src/lib/mina_base/parties.ml:44:8
pub struct MinaBasePartiesTStableV1OtherPartiesArg0Arg0 {
    pub party: MinaBasePartyTWireStableV1,
    pub party_digest: (),
    pub calls: Vec<MinaBasePartiesTStableV1OtherPartiesArg0>,
}

/// Location: src/lib/mina_base/with_stack_hash.ml:6:4
pub struct MinaBasePartiesTStableV1OtherPartiesArg0 {
    pub elt: MinaBasePartiesTStableV1OtherPartiesArg0Arg0,
    pub stack_hash: (),
}

/// Origin: Mina_base__Parties.T.Stable.V1.t
/// Location: src/lib/mina_base/parties.ml:824:12
pub struct MinaBasePartiesTStableV1 {
    pub fee_payer: MinaBasePartyFeePayerStableV1,
    pub other_parties: Vec<MinaBasePartiesTStableV1OtherPartiesArg0>,
    pub memo: MinaBaseSignedCommandMemoStableV1,
}

/// Origin: Mina_base__User_command.Stable.V2.t
/// Location: src/lib/mina_base/user_command.ml:64:4
/// Location: src/lib/mina_base/user_command.ml:7:6
pub enum MinaBaseUserCommandStableV2 {
    SignedCommand(MinaBaseSignedCommandStableV2),
    Parties(MinaBasePartiesTStableV1),
}

/// Origin: Mina_base__Transaction_status.Stable.V2.t
/// Location: src/lib/mina_base/transaction_status.ml:423:4
pub enum MinaBaseTransactionStatusStableV2 {
    Applied,
    Failed(MinaBaseTransactionStatusFailureCollectionStableV1),
}

/// Location: src/lib/mina_base/with_status.ml:6:4
pub struct StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2Arg1 {
    pub data: MinaBaseUserCommandStableV2,
    pub status: MinaBaseTransactionStatusStableV2,
}

/// Origin: Mina_base__Coinbase_fee_transfer.Stable.V1.t
/// Location: src/lib/mina_base/coinbase_fee_transfer.ml:7:4
pub struct MinaBaseCoinbaseFeeTransferStableV1 {
    pub receiver_pk: NonZeroCurvePointUncompressedStableV1,
    pub fee: CurrencyFeeStableV1,
}

/// Origin: Staged_ledger_diff__Diff.Ft.Stable.V1.t
/// Location: src/lib/staged_ledger_diff/diff.ml:66:6
pub type StagedLedgerDiffDiffFtStableV1 = MinaBaseCoinbaseFeeTransferStableV1;

/// Location: src/lib/staged_ledger_diff/diff.ml:10:6
pub enum StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2Coinbase {
    Zero,
    One(Option<StagedLedgerDiffDiffFtStableV1>),
    Two(
        Option<(
            StagedLedgerDiffDiffFtStableV1,
            Option<StagedLedgerDiffDiffFtStableV1>,
        )>,
    ),
}

/// Origin: Staged_ledger_diff__Diff.Pre_diff_with_at_most_two_coinbase.Stable.V2.t
/// Location: src/lib/staged_ledger_diff/diff.ml:140:6
/// Location: src/lib/staged_ledger_diff/diff.ml:82:6
pub struct StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2 {
    pub completed_works: Vec<TransactionSnarkWorkTStableV2>,
    pub commands: Vec<StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2Arg1>,
    pub coinbase: StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2Coinbase,
}

/// Location: src/lib/staged_ledger_diff/diff.ml:43:6
pub enum StagedLedgerDiffDiffPreDiffWithAtMostOneCoinbaseStableV2Coinbase {
    Zero,
    One(Option<StagedLedgerDiffDiffFtStableV1>),
}

/// Origin: Staged_ledger_diff__Diff.Pre_diff_with_at_most_one_coinbase.Stable.V2.t
/// Location: src/lib/staged_ledger_diff/diff.ml:159:6
/// Location: src/lib/staged_ledger_diff/diff.ml:111:6
pub struct StagedLedgerDiffDiffPreDiffWithAtMostOneCoinbaseStableV2 {
    pub completed_works: Vec<TransactionSnarkWorkTStableV2>,
    pub commands: Vec<StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2Arg1>,
    pub coinbase: StagedLedgerDiffDiffPreDiffWithAtMostOneCoinbaseStableV2Coinbase,
}

/// Origin: Staged_ledger_diff__Diff.Diff.Stable.V2.t
/// Location: src/lib/staged_ledger_diff/diff.ml:178:6
pub type StagedLedgerDiffDiffDiffStableV2 = (
    StagedLedgerDiffDiffPreDiffWithAtMostTwoCoinbaseStableV2,
    Option<StagedLedgerDiffDiffPreDiffWithAtMostOneCoinbaseStableV2>,
);

/// Origin: Staged_ledger_diff__Diff.Stable.V2.t
/// Location: src/lib/staged_ledger_diff/diff.ml:195:4
pub struct StagedLedgerDiffDiffStableV2 {
    pub diff: StagedLedgerDiffDiffDiffStableV2,
}

/// Origin: Staged_ledger_diff__Body.Stable.V1.t
/// Location: src/lib/staged_ledger_diff/body.ml:12:4
pub struct StagedLedgerDiffBodyStableV1 {
    pub staged_ledger_diff: StagedLedgerDiffDiffStableV2,
}

/// Origin: Transaction_snark.Statement.Stable.V2.t
/// Location: src/lib/transaction_snark/transaction_snark.ml:205:6
/// Location: src/lib/transaction_snark/transaction_snark.ml:122:8
pub struct TransactionSnarkStatementStableV2 {
    pub source: MinaStateBlockchainStateValueStableV2Registers,
    pub target: MinaStateBlockchainStateValueStableV2Registers,
    pub supply_increase: CurrencyAmountMakeStrStableV1,
    pub fee_excess: MinaBaseFeeExcessStableV1,
    pub sok_digest: (),
}

/// Origin: Transaction_snark_work.Statement.Stable.V2.t
/// Location: src/lib/transaction_snark_work/transaction_snark_work.ml:23:6
/// Location: src/lib/one_or_two/one_or_two.ml:7:4
pub enum TransactionSnarkWorkStatementStableV2 {
    One(TransactionSnarkStatementStableV2),
    Two(
        (
            TransactionSnarkStatementStableV2,
            TransactionSnarkStatementStableV2,
        ),
    ),
}

/// Origin: Mina_base__Fee_with_prover.Stable.V1.t
/// Location: src/lib/mina_base/fee_with_prover.ml:7:4
pub struct MinaBaseFeeWithProverStableV1 {
    pub fee: CurrencyFeeStableV1,
    pub prover: NonZeroCurvePointUncompressedStableV1,
}

/// Location: src/lib/network_pool/priced_proof.ml:9:4
pub struct NetworkPoolSnarkPoolDiffVersionedStableV2AddSolvedWork1 {
    pub proof: TransactionSnarkWorkStatementStableV2,
    pub fee: MinaBaseFeeWithProverStableV1,
}
