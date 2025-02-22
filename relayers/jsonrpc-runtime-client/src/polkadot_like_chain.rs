use relay_substrate_client::{Chain, ChainBase};
use std::time::Duration;
use t3rn_primitives::bridges::polkadot_core as bp_polkadot_core;

/// Polkadot header id.
// pub type HeaderId = relay_utils::HeaderId<bp_polkadot_core::Hash, bp_polkadot_core::BlockNumber>;
/// Polkadot header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_polkadot_core::Header>;

/// Polkadot chain definition
#[derive(Debug, Clone, Copy)]
pub struct Rococo;

impl ChainBase for Rococo {
    type BlockNumber = bp_polkadot_core::BlockNumber;
    type Hash = bp_polkadot_core::Hash;
    type Hasher = bp_polkadot_core::Hasher;
    type Header = bp_polkadot_core::Header;
}

impl Chain for Rococo {
    type AccountId = bp_polkadot_core::AccountId;
    type Call = ();
    type Index = bp_polkadot_core::Nonce;
    type SignedBlock = bp_polkadot_core::SignedBlock;

    const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);
    const NAME: &'static str = "Polkadot";
}
