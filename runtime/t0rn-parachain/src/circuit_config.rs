use crate::{
    AccountId, AccountManager, Balances, BlockNumber, Call, Circuit, Clock, Event, Portal, Runtime,
    Timestamp, XDNS,
};

use sp_std::vec;

use frame_support::{parameter_types, traits::ConstU32, weights::Weight, PalletId};
use pallet_grandpa_finality_verifier::bridges::runtime as bp_runtime;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, Convert, One},
    Perbill,
};

pub type RococoLightClient = ();
pub type PolkadotLightClient = pallet_grandpa_finality_verifier::Instance1;
pub type KusamaLightClient = pallet_grandpa_finality_verifier::Instance2;

impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

pub struct GlobalOnInitQueues;

impl pallet_clock::traits::OnHookQueues<Runtime> for GlobalOnInitQueues {
    fn process(n: BlockNumber, on_init_weight_limit: Weight) -> Weight {
        let mut weights_consumed = vec![];
        const PROCESS_SIGNAL_SHARE: u32 = 15;
        const XTX_TICK_SHARE: u32 = 35;
        const REVERT_XTX_SHARE: u32 = 35;
        const BUMP_ROUND_SHARE: u32 = 5;
        const CALC_CLAIMABLE_SHARE: u32 = 10;
        if PROCESS_SIGNAL_SHARE
            + XTX_TICK_SHARE
            + REVERT_XTX_SHARE
            + BUMP_ROUND_SHARE
            + CALC_CLAIMABLE_SHARE
            > 100
        {
            log::error!(
                "GlobalOnInitQueues::Invalid shares exceed 100%, returning 0 - re-check the shares"
            );
            return 0
        }
        // Iterate over all pre-init hooks implemented by pallets and return aggregated weight
        weights_consumed.push(Circuit::process_signal_queue(
            n,
            BlockNumber::one(),
            Perbill::from_percent(PROCESS_SIGNAL_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Circuit::process_signal_queue consumed: {:?}",
            weights_consumed
                .last()
                .expect("Circuit::process_signal_queue consumed weight")
        );
        weights_consumed.push(Circuit::process_xtx_tick_queue(
            n,
            BlockNumber::one(),
            Perbill::from_percent(XTX_TICK_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Circuit::process_xtx_tick_queue consumed: {:?}",
            weights_consumed
                .last()
                .expect("Circuit::process_xtx_tick_queue consumed weight")
        );
        weights_consumed.push(Circuit::process_revert_xtx_queue(
            n,
            10u32,
            Perbill::from_percent(REVERT_XTX_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Circuit::process_revert_xtx_queue consumed: {:?}",
            weights_consumed
                .last()
                .expect("Circuit::process_revert_xtx_queue consumed weight")
        );
        weights_consumed.push(Clock::check_bump_round(
            n,
            BlockNumber::one(),
            Perbill::from_percent(BUMP_ROUND_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Clock::check_bump_round consumed: {:?}",
            weights_consumed
                .last()
                .expect("Clock::check_bump_round consumed weight")
        );
        weights_consumed.push(Clock::calculate_claimable_for_round(
            n,
            BlockNumber::one(),
            Perbill::from_percent(CALC_CLAIMABLE_SHARE) * on_init_weight_limit,
        ));
        log::debug!(
            "Clock::calculate_claimable_for_round consumed: {:?}",
            weights_consumed
                .last()
                .expect("Clock::calculate_claimable_for_round consumed weight")
        );
        let total_consumed: Weight = weights_consumed
            .iter()
            .fold(0, |acc: Weight, weight: &Weight| {
                acc.saturating_add(*weight)
            });

        log::debug!(
            "Total weight consumed by on init hook: {:?}",
            total_consumed
        );

        total_consumed
    }
}

impl pallet_clock::Config for Runtime {
    type AccountManager = AccountManager;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type OnFinalizeQueues = pallet_clock::traits::EmptyOnHookQueues<Self>;
    type OnInitializeQueues = GlobalOnInitQueues;
    type RoundDuration = ConstU32<500u32>;
}

impl pallet_xdns::Config for Runtime {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type Time = Timestamp;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Balances = Balances;
    type Currency = Balances;
    type Event = Event;
    type WeightInfo = pallet_contracts_registry::weights::SubstrateWeight<Runtime>;
}

impl pallet_portal::Config for Runtime {
    type Event = Event;
    type WeightInfo = pallet_portal::weights::SubstrateWeight<Runtime>;
    type Xdns = XDNS;
}

parameter_types! {
    pub const PortalPalletId: PalletId = PalletId(*b"pal/port");
}
pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

parameter_types! {
    pub const CircuitAccountId: AccountId = AccountId::new([51u8; 32]); // 0x333...3
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

impl pallet_circuit::Config for Runtime {
    type AccountManager = AccountManager;
    type Balances = Balances;
    type Call = Call;
    type Currency = Balances;
    type DeletionQueueLimit = ConstU32<100u32>;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Portal = Portal;
    type SFXBiddingPeriod = ConstU32<3u32>;
    type SelfAccountId = CircuitAccountId;
    type SelfGatewayId = SelfGatewayId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<5u32>;
    type WeightInfo = ();
    // type XBIPortal = XBIPortalRuntimeEntry;
    // type XBIPromise = XBIPortal;
    type Xdns = XDNS;
    type XtxTimeoutCheckInterval = ConstU32<10u32>;
    type XtxTimeoutDefault = ConstU32<400u32>;
}

parameter_types! {
    pub const HeadersToStore: u32 = 100;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

impl pallet_grandpa_finality_verifier::Config<RococoLightClient> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type HeadersToStore = HeadersToStore;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<PolkadotLightClient> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type HeadersToStore = HeadersToStore;
    type WeightInfo = ();
}

impl pallet_grandpa_finality_verifier::Config<KusamaLightClient> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type HeadersToStore = HeadersToStore;
    type WeightInfo = ();
}
