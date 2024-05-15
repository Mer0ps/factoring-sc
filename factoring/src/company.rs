multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const DEFAULT_PERCENT_FEE: u64 = 100; //1%

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Company<M: ManagedTypeApi> {
 pub id_offchain: u64,
 pub administrators: ManagedVec<M, ManagedAddress<M>>,
 pub is_kyc: bool,
 pub fee: u64,
 pub withdraw_address: ManagedAddress<M>,
 pub reliability_score: u8
}