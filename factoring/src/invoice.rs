multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    PendingValidation = 0,
    Valid = 1,
    PartiallyFunded = 2,
    Payed = 3,
    FullyFunded = 4,
    Refused = 5,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Invoice<M: ManagedTypeApi> {
 pub hash: ManagedBuffer<M>,
 pub amount: BigUint<M>,
 pub identifier: EgldOrEsdtTokenIdentifier<M>,
 pub status: Status,
 pub issue_date: u64,
 pub due_date: u64,
 pub euribor_rate: u8,
 pub payed_date: Option<u64>
}