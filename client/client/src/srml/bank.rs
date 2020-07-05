use crate::srml::org::{
    Org,
    OrgEventsDecoder,
};
use codec::{
    Codec,
    Decode,
    Encode,
};
use frame_support::Parameter;
use sp_runtime::traits::{
    AtLeast32Bit,
    MaybeSerializeDeserialize,
    Member,
    Zero,
};
use std::fmt::Debug;
use substrate_subxt::system::{
    System,
    SystemEventsDecoder,
};
use util::bank::{
    BankState,
    OnChainTreasuryID,
};

pub type BalanceOf<T> = <T as Bank>::Currency; // as Currency<<T as System>::AccountId>>::Balance;

/// The subset of the bank trait and its inherited traits that the client must inherit
#[module]
pub trait Bank: System + Org {
    /// The currency type for on-chain transactions
    type Currency: Parameter
        + Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + PartialOrd
        + PartialEq
        + Zero; // + Currency<<Self as System>::AccountId> // commented out until #93 is resolved
}

// ~~ Values (Constants) ~~

#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct MinimumInitialDepositStore<T: Bank> {
    pub amount: BalanceOf<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct MinimumTransferStore<T: Bank> {
    pub amount: BalanceOf<T>,
}

// ~~ Maps ~~

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct BankStoresStore<T: Bank> {
    #[store(returns = BankState<<T as System>::AccountId, <T as Org>::OrgId>)]
    pub id: OnChainTreasuryID,
    phantom: std::marker::PhantomData<T>,
}

// ~~ (Calls, Events) ~~

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RegisterAndSeedForBankAccountCall<T: Bank> {
    pub seed: BalanceOf<T>,
    pub hosting_org: <T as Org>::OrgId,
    pub bank_operator: Option<<T as Org>::OrgId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RegisteredNewOnChainBankEvent<T: Bank> {
    pub seeder: <T as System>::AccountId,
    pub new_bank_id: OnChainTreasuryID,
    pub seed: BalanceOf<T>,
    pub hosting_org: <T as Org>::OrgId,
    pub bank_operator: Option<<T as Org>::OrgId>,
}