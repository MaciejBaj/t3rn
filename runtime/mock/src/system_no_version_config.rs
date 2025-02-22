use crate::{accounts_config::AccountManagerCurrencyAdapter, Assets, Hash as HashPrimitive, *};
use circuit_runtime_pallets::pallet_asset_tx_payment::HandleCredit;
use frame_support::{
    parameter_types,
    traits::{
        fungibles::{Balanced, CreditOf},
        ConstU32, ConstU8, NeverEnsureOrigin,
    },
    weights::{constants::RocksDbWeight, ConstantMultiplier, IdentityFee},
    PalletId,
};
use frame_system::EnsureRoot;
use sp_runtime::{
    generic,
    traits::{AccountIdLookup, BlakeTwo256, ConvertInto},
    Permill,
};

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The ubiquitous event type.
    type Event = Event;
    /// The type for hashing blocks and tries.
    type Hash = HashPrimitive;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Version of the runtime.
    type Version = ();
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    type MinimumPeriod = MinimumPeriod;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1_u128;
}

impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    /// The ubiquitous event type.
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type Event = Event;
    type FeeMultiplierUpdate = ();
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type OnChargeTransaction = AccountManagerCurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
}

impl pallet_authorship::Config for Runtime {
    type EventHandler = ();
    type FilterUncle = ();
    type FindAuthor = ();
    type UncleGenerations = ();
}

/// A `HandleCredit` implementation that transfers 80% of the fees to the
/// block author and 20% to the treasury. Will drop and burn the assets
/// in case the transfer fails.
pub struct CreditToBlockAuthor;
impl HandleCredit<AccountId, Assets> for CreditToBlockAuthor {
    fn handle_credit(credit: CreditOf<AccountId, Assets>) {
        if let Some(author) = pallet_authorship::Pallet::<Runtime>::author() {
            let author_credit = credit
                .peek()
                .saturating_mul(80_u32.into())
                .saturating_div(<u32 as Into<Balance>>::into(100_u32));
            let (author_cut, treasury_cut) = credit.split(author_credit);
            // Drop the result which will trigger the `OnDrop` of the imbalance in case of error.
            Assets::resolve(&author, author_cut);
            Assets::resolve(&Treasury::account_id(), treasury_cut);
        }
    }
}

impl pallet_asset_tx_payment::Config for Runtime {
    type Fungibles = Assets;
    type OnChargeAssetTransaction = pallet_asset_tx_payment::FungiblesAdapter<
        pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
        CreditToBlockAuthor,
    >;
}

parameter_types! {
    pub const TreasuryId: PalletId = PalletId(*b"pottrsry");
    pub const MaxApprovals: u32 = 10;
    pub const ProposalBond: Permill = Permill::from_percent(1);
    pub const SpendPeriod: u32 = 60 / 12;
    pub const ProposalBondMinimum: u128 = 1_000_000_000_000_u128;
}

impl pallet_treasury::Config for Runtime {
    type ApproveOrigin = EnsureRoot<AccountId>;
    type Burn = ();
    type BurnDestination = ();
    type Currency = Balances;
    type Event = Event;
    type MaxApprovals = MaxApprovals;
    type OnSlash = Treasury;
    type PalletId = TreasuryId;
    type ProposalBond = ProposalBond;
    type ProposalBondMaximum = ();
    type ProposalBondMinimum = ProposalBondMinimum;
    type RejectOrigin = EnsureRoot<AccountId>;
    type SpendFunds = ();
    type SpendOrigin = NeverEnsureOrigin<Balance>;
    type SpendPeriod = SpendPeriod;
    type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
    type Call = Call;
    type Event = Event;
}

impl pallet_utility::Config for Runtime {
    type Call = Call;
    type Event = Event;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    Call: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}
