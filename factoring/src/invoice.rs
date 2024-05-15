multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    PendingValidation = 0,
    Valid = 1,
    Funded = 2,
    Payed = 3,
    Refused = 4,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Invoice<M: ManagedTypeApi> {
 pub hash: ManagedBuffer<M>,
 pub amount: BigUint<M>,
 pub identifier: EgldOrEsdtTokenIdentifier<M>,
 pub status: Status,
 pub due_date: u64,
 pub payed_date: Option<u64>
}