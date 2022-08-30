#[cfg(feature = "hashing")]
mod utils;

#[cfg(feature = "hashing")]
mod tests {
    use super::*;
    use mina_p2p_messages::p2p::MinaBlockExternalTransitionRawVersionedStable;

    #[test]
    fn state_hash_1() {
        let encoded = utils::read("external-transition/1.bin").unwrap();
        let mut ptr = encoded.as_slice();
        let external_transition: MinaBlockExternalTransitionRawVersionedStable =
            binprot::BinProtRead::binprot_read(&mut ptr).unwrap();
        let json = serde_json::to_string_pretty(&external_transition).unwrap();
        eprintln!("{json}");
        let mut hasher = mina_hasher::create_legacy(());
        let hash = external_transition.inner().protocol_state.hash(&mut hasher);
        assert_eq!(
            hash.to_string(),
            "3NLUiFWnakJxjUhFoNw1PPSRuRgJbhmfHqSLweEWrxcPpo8ikVTb".to_owned()
        );
    }
}
