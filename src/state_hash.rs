use std::fmt;

pub use super::p2p::{StateHashStable, StateHashStableV1};

impl fmt::Display for StateHashStable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = vec![];
        binprot::BinProtWrite::binprot_write(self, &mut buf).or(Err(fmt::Error))?;

        bs58::encode(&buf)
            .with_check_version(0x10)
            .into_string()
            .fmt(f)
    }
}
