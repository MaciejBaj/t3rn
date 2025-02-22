use crate::{
    AccountId, AssetId, Assets, Balance, Balances, Call, Clock, EnsureRoot, Event, Imbalance,
    OnUnbalanced, Runtime, ThreeVm, Timestamp,
};
use frame_support::parameter_types;
use sp_core::crypto::AccountId32;
use sp_runtime::traits::ConvertInto;

parameter_types! {
    // TODO: update me to be better
    pub EscrowAccount: AccountId32 = AccountId32::new([51_u8; 32]);
}

pallet_account_manager::setup_currency_adapter!();

impl pallet_account_manager::Config for Runtime {
    type AssetBalanceOf = ConvertInto;
    type AssetId = AssetId;
    type Assets = Assets;
    type Clock = Clock;
    type Currency = Balances;
    type EscrowAccount = EscrowAccount;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type Time = Timestamp;
    type WeightInfo = ();
}

parameter_types! {
    pub const AssetDeposit: Balance = 0; // 1 UNIT deposit to create asset
    pub const ApprovalDeposit: Balance = 0;
    pub const AssetsStringLimit: u32 = 50;
    /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
    // https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
    pub const AssetAccountDeposit: Balance = 0;
}

impl pallet_assets::Config for Runtime {
    type ApprovalDeposit = ApprovalDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type AssetDeposit = AssetDeposit;
    type AssetId = circuit_runtime_types::AssetId;
    type Balance = Balance;
    type Currency = Balances;
    type Event = Event;
    type Extra = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type Freezer = ();
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type StringLimit = AssetsStringLimit;
    type WeightInfo = ();
}

parameter_types! {
    pub const RegCost: u128 = 100_000_000_000;
}

impl pallet_asset_registry::Config for Runtime {
    type Assets = Assets;
    type Call = Call;
    type Currency = Balances;
    type Event = Event;
    type RegistrationCost = RegCost;
}
