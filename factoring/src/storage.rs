
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{company::Company, invoice::Invoice, contract::CustomerContract};

#[multiversx_sc::module]
pub trait Storage {

    #[view(getCompany)]
    #[storage_mapper("company")]
    fn companies(&self, id: &u64) -> SingleValueMapper<Company<Self::Api>>;

    #[storage_mapper("company_count")]
    fn company_count(&self) -> SingleValueMapper<u64>;

    #[view(getCustomerContract)]
    #[storage_mapper("customer_contract")]
    fn customer_contracts(&self, id: &u64) -> SingleValueMapper<CustomerContract>;

    #[storage_mapper("customer_contract_count")]
    fn customer_contract_count(&self) -> SingleValueMapper<u64>;

    #[view(getInvoicesByContract)]
    #[storage_mapper("invoices_by_contract")]
    fn invoices_by_contract(&self, id_contract: &u64) -> VecMapper<Invoice<Self::Api>>;

    #[view(getContractsClientByAccount)]
    #[storage_mapper("contracts_client_by_account")]
    fn contracts_client_by_account(&self, id_account: &u64) -> UnorderedSetMapper<u64>;

    #[view(getAvailableAssetByAccountAndIdentifier)]
    #[storage_mapper("assets_by_account_and_identifier")]
    fn assets_by_account_and_identifier(&self, id_account: &u64, identifier: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getFundsByAccount)]
    #[storage_mapper("funds_by_account")]
    fn funds_by_account(&self, id_account: &u64, identifier: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getHatomControllerAddress)]
    #[storage_mapper("hatomControllerAddress")]
    fn hatom_controller_address(&self) -> SingleValueMapper<Self::Api, ManagedAddress>;
}