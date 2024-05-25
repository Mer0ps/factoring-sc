#![no_std]

use multiversx_sc::imports::*;

mod errors;
mod storage;
mod company;
mod admin_config;
mod invoice;
mod contract;
mod events;
mod stable_farming;
mod hatom_proxy;
mod account_manager;

use crate::{invoice::*, errors::*, company::*};

pub const DEFAULT_PERCENT_PAY: u64 = 7_000; //70%
pub const ONE_HUNDRED_PERCENT: u64 = 10_000; //100%
pub const MIN_SCORE: u8 = 80;

#[multiversx_sc::contract]
pub trait Factoring :
    admin_config::AdminConfigModule   
    + storage::Storage
    + events::EventsModule
    + stable_farming::StableFarmingModule
    + account_manager::AccountManagerModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(addInvoice)]
    fn add_invoice(&self, id_contract: u64, hash: ManagedBuffer, amount: BigUint, issue_date: u64, due_date: u64, token_identifier: EgldOrEsdtTokenIdentifier){
        let caller = self.blockchain().get_caller();
        self.allowed_tokens().require_whitelisted(&token_identifier);
        let contract = self.customer_contracts(&id_contract).get();
        self.require_valid_administrator(contract.id_supplier, &caller);

        let (rate, _timestamp) = self.euribor_rate().get();

        let new_invoice = Invoice {
            hash: hash.clone(),
            amount: amount.clone(),
            identifier: token_identifier,
            status: Status::PendingValidation,
            issue_date: issue_date,
            due_date: due_date,
            euribor_rate: rate,
            payed_date: Option::None
        };

        self.invoices_by_contract(&id_contract).push(&new_invoice);
        
        let invoice_id = self.invoices_by_contract(&id_contract).len() as u64;

        self.invoice_add_event(id_contract, hash.clone(), amount.clone(), due_date, invoice_id, self.blockchain().get_block_timestamp());
    }

    #[endpoint(confirmInvoice)]
    fn confirm_invoice(&self, id_contract: u64, id_invoice: u64, status: Status){
        let caller = self.blockchain().get_caller();
        let contract = self.customer_contracts(&id_contract).get();
        self.require_valid_administrator(contract.id_client, &caller);

        let client = self.companies(&contract.id_client).get();
        require!(client.reliability_score > MIN_SCORE, INSUFFICIENT_SCORE);

        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);

        require!(invoice.status == Status::PendingValidation, INVOICE_CONFIRM_WRONG_STATUS);

        invoice.status = status;
        self.invoices_by_contract(&id_contract).set(id_invoice as usize, &invoice);
        self.invoice_confirm_event(id_contract, id_invoice, status, self.blockchain().get_block_timestamp());
    }

    #[endpoint(fundInvoice)]
    fn fund_invoice(&self, id_contract: u64, id_invoice: u64){
        self.require_caller_is_admin();

        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);

        require!(invoice.status == Status::Valid, INVOICE_NOT_FUNDABLE);

        let contract = self.customer_contracts(&id_contract).get();
        let company = self.companies(&contract.id_supplier).get();
        
        
        let funds = self.get_current_funds(&invoice.identifier);

        require!(funds >= invoice.amount, NOT_ENOUGH_FUNDS);

        let percent_to_pay = BigUint::from(DEFAULT_PERCENT_PAY) * invoice.amount.clone() / BigUint::from(ONE_HUNDRED_PERCENT);

        //direct_non_zero
        self.tx()
            .to(&company.withdraw_address)
            .egld_or_single_esdt(&invoice.identifier, 0, &percent_to_pay)
            .transfer_if_not_empty();

        invoice.status = Status::PartiallyFunded;
        self.invoices_by_contract(&id_contract).set(id_invoice as usize, &invoice);
        self.invoice_fund_event(id_contract, id_invoice, self.blockchain().get_block_timestamp());
    }

    #[payable("*")]
    #[endpoint(payInvoice)]
    fn pay_invoice(&self, id_contract: u64, id_invoice: u64){
        let caller = self.blockchain().get_caller();
        let contract = self.customer_contracts(&id_contract).get();
        self.require_valid_administrator(contract.id_client, &caller);

        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);

        require!(invoice.status == Status::PartiallyFunded, INVOICE_NOT_PAYABLE);
        
        
        let (payment_token, _, payment_amount) = self.call_value().egld_or_single_esdt().into_tuple();
        require!(payment_token == invoice.identifier, WRONG_TOKEN_ID);
        require!(payment_amount >= invoice.amount, NOT_ENOUGH_FUNDS);

        //Send back the difference
        if payment_amount > invoice.amount {
            let change = &payment_amount - &invoice.amount;

            self.tx()
                .to(&self.blockchain().get_caller())
                .egld_or_single_esdt(&invoice.identifier, 0, &change)
                .transfer();

        }

        self.pay_invoice_fn(&mut invoice, id_contract, id_invoice);
    }

    #[endpoint(payInvoiceAuto)]
    fn pay_invoice_auto(&self, id_contract: u64, id_invoice: u64){
        self.require_caller_is_admin();
        
        let contract = self.customer_contracts(&id_contract).get();
        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);
        
        let available_funds = self.funds_by_account(&contract.id_client, &invoice.identifier).get();
        
        require!(available_funds >= invoice.amount, NOT_ENOUGH_FUNDS);

        self.pay_invoice_fn(&mut invoice, id_contract, id_invoice);

        self.funds_by_account(&contract.id_client, &invoice.identifier).update(|val| *val -= invoice.amount);
    }

    fn pay_invoice_fn(&self, invoice: &mut Invoice<Self::Api>, id_contract: u64, id_invoice: u64){

        // let already_paid = BigUint::from(DEFAULT_PERCENT_PAY) * invoice.amount.clone() / BigUint::from(ONE_HUNDRED_PERCENT);
        // let remaining_amount = invoice.amount.clone() - already_paid;
        // let fees = BigUint::from(DEFAULT_PERCENT_FEE) * invoice.amount.clone() / BigUint::from(ONE_HUNDRED_PERCENT);
        // let amount_to_send = remaining_amount - fees;
        let current_timestamp = self.blockchain().get_block_timestamp();

        // let company = self.companies(&contract.id_supplier).get();

        // self.tx()
        //     .to(&company.withdraw_address)
        //     .egld_or_single_esdt(&invoice.identifier, 0, &amount_to_send)
        //     .transfer_if_not_empty();

        invoice.status = Status::Payed;
        invoice.payed_date = Option::Some(current_timestamp);
        self.invoices_by_contract(&id_contract).set(id_invoice as usize, &invoice);

        self.invoice_pay_event(id_contract, id_invoice, current_timestamp);        
    }


    #[endpoint(fundRemainingAmount)]
    fn fund_remaining_amount(&self, id_contract: u64, id_invoice: u64){

        self.require_caller_is_admin();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);

        require!(invoice.status == Status::Payed, INVOICE_NOT_FULLY_FUNDABLE);
        //require!(invoice.due_date < current_timestamp, INVOICE_DUE_DATE_NOT_REACHED);

        let already_paid = BigUint::from(DEFAULT_PERCENT_PAY) * invoice.amount.clone() / BigUint::from(ONE_HUNDRED_PERCENT);
        let remaining_amount = invoice.amount.clone() - already_paid;
        let total_fees = self.calculate_total_fees(&invoice);
        let amount_to_send = remaining_amount - total_fees;
        
        let contract = self.customer_contracts(&id_contract).get();
        let company = self.companies(&contract.id_supplier).get();

        self.tx()
            .to(&company.withdraw_address)
            .egld_or_single_esdt(&invoice.identifier, 0, &amount_to_send)
            .transfer_if_not_empty();

        invoice.status = Status::FullyFunded;
        self.invoices_by_contract(&id_contract).set(id_invoice as usize, &invoice);

        self.invoice_fully_fund_event(id_contract, id_invoice, current_timestamp);        
    }

    #[endpoint(calculateReliabilityScore)]
    fn calculate_reliability_score(&self, id_account: u64) {

        let mut total_reliability_score = 100;
        let current_timestamp = self.blockchain().get_block_timestamp();

        'client_loop:
        for id_contract in self.contracts_client_by_account(&id_account).iter() {
            for invoice in self.invoices_by_contract(&id_contract).iter() {
                let is_over_due = self.is_invoice_overdue(invoice.due_date, invoice.payed_date, current_timestamp);
                if is_over_due {
                    total_reliability_score -= 1;

                    if total_reliability_score == 0 {
                        break 'client_loop;
                    }
                }
            }
        }
        self.companies(&id_account).update(|val| val.reliability_score = total_reliability_score as u8);
        self.company_new_score_event(id_account, total_reliability_score);
    }

    fn is_invoice_overdue(&self, due_date: u64, opt_payed_date: Option<u64>, current_timestamp: u64) -> bool {
        match opt_payed_date {
            Option::Some(payed_date) => {
                if current_timestamp > due_date && payed_date > due_date {
                    true
                }else{
                    false
                }
            },
            Option::None => {
                if current_timestamp > due_date {
                    true
                }else{
                    false
                }
            }
        }
    }

    fn calculate_financing_fees(&self, amount: &BigUint, due_date: u64, issue_date: u64, euribor_rate: u32) -> BigUint {
        let duration_seconds = due_date - issue_date;
        let duration_days = duration_seconds / (24 * 60 * 60); 

        let total_rate = BigUint::from(euribor_rate) * BigUint::from(duration_days);

        let financing_fees = amount * &total_rate / (BigUint::from(365u32) * BigUint::from(ONE_HUNDRED_PERCENT));

        financing_fees
    }

    fn calculate_commission(&self, amount: &BigUint) -> BigUint {
        BigUint::from(DEFAULT_PERCENT_FEE) * amount / BigUint::from(ONE_HUNDRED_PERCENT)
    }

    fn calculate_total_fees(&self, invoice: &Invoice<Self::Api>) -> BigUint {
        let commission = self.calculate_commission(&invoice.amount);
        let financing_fees = self.calculate_financing_fees(&invoice.amount, invoice.due_date, invoice.issue_date, invoice.euribor_rate);
        commission + financing_fees
    }
}
