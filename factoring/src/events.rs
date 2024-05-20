multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{invoice::Status};

#[multiversx_sc::module]
pub trait EventsModule {

    #[event("invoice_add_event")]
    fn invoice_add_event(
        &self,
        #[indexed] id_contract: u64,
        #[indexed] hash: ManagedBuffer,
        #[indexed] amount: BigUint,
        #[indexed] due_date: u64,
        #[indexed] invoice_id: u64,
        #[indexed] timestamp: u64,
    );

    #[event("invoice_confirm_event")]
    fn invoice_confirm_event(
        &self,
        #[indexed] id_contract: u64,
        #[indexed] id_invoice: u64,
        #[indexed] status: Status,
        #[indexed] timestamp: u64,
    );

    #[event("invoice_fund_event")]
    fn invoice_fund_event(
        &self,
        #[indexed] id_contract: u64,
        #[indexed] id_invoice: u64,
        #[indexed] timestamp: u64,
    );

    #[event("invoice_pay_event")]
    fn invoice_pay_event(
        &self,
        #[indexed] id_contract: u64,
        #[indexed] id_invoice: u64,
        #[indexed] pay_date: u64,
    );

    #[event("contract_create_event")]
    fn contract_create_event(
        &self,
        #[indexed] id_supplier: u64,
        #[indexed] id_client: u64,
        #[indexed] id_contract: u64,
    );

    #[event("contract_sign_event")]
    fn contract_sign_event(
        &self,
        #[indexed] id_contract: u64,
    );

    #[event("company_create_event")]
    fn company_create_event(
        &self,
        #[indexed] id_offchain: u64,
        #[indexed] id_company: u64,
        #[indexed] score: u8,
        #[indexed] fee: u64,
    );

    #[event("company_new_score_event")]
    fn company_new_score_event(
        &self,
        #[indexed] id_company: u64,
        #[indexed] score: u8,
    );

    #[event("company_add_admin_event")]
    fn company_add_admin_event(
        &self,
        #[indexed] id_offchain: u64,
        #[indexed] address: ManagedAddress,
    );

    #[event("company_add_funds_event")]
    fn company_add_funds_event(
        &self,
        #[indexed] id_company: u64,
    );

    #[event("sc_add_admin_event")]
    fn sc_add_admin_event(
        &self,
        #[indexed] address: ManagedAddress,
    );

    #[event("sc_add_funds_event")]
    fn sc_add_funds_event(
        &self,
    );
}