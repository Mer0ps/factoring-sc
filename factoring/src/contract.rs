multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct CustomerContract {
 pub id_supplier: u64,
 pub id_client: u64,
 pub is_signed: bool
}