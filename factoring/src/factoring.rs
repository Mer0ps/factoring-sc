#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod errors;
mod storage;
mod company;
mod admin_whitelist;
mod invoice;
mod contract;
mod events;
mod stable_farming;
mod hatom_proxy;

use crate::{invoice::*, contract::CustomerContract};

use crate::errors::*;
use crate::company::*;

pub const DEFAULT_PERCENT_PAY: u64 = 7_000; //70%
pub const ONE_HUNDRED_PERCENT: u64 = 10_000; //100%
pub const DEFAULT_SCORE: u8 = 100;
pub const MIN_SCORE: u8 = 80;

#[multiversx_sc::contract]
pub trait Factoring :
    admin_whitelist::AdminWhitelistModule   
    + storage::Storage
    + events::EventsModule
    + stable_farming::StableFarmingModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(addCompany)]
    fn add_company(&self, id_offchain: u64, administrators: ManagedVec<ManagedAddress>, is_kyc: bool, withdraw_address: ManagedAddress){
        self.require_caller_is_admin();
        let id_company = self.company_count().get();

        let new_company = Company {
            id_offchain: id_offchain,
            administrators: administrators,
            is_kyc: is_kyc,
            fee: DEFAULT_PERCENT_FEE,
            withdraw_address: withdraw_address,
            reliability_score: DEFAULT_SCORE
        };

        self.company_count().set(id_company + 1);
        self.companies(&id_company).set(new_company);
        self.company_create_event(id_offchain, id_company, DEFAULT_SCORE, DEFAULT_PERCENT_FEE);
    }

    #[endpoint(createFactoringContract)]
    fn create_factoring_contract(&self, id_supplier: u64, id_client: u64) {
        let caller = self.blockchain().get_caller();
        self.require_valid_administrator(id_supplier, &caller);

        let new_contract = CustomerContract {
            id_supplier: id_supplier,
            id_client: id_client,
            is_signed: false
        };

        let id_contract = self.customer_contract_count().get();

        self.customer_contract_count().set(id_contract + 1);
        self.customer_contracts(&id_contract).set(new_contract);

        self.contract_create_event(id_supplier, id_client, id_contract);
    }

    #[endpoint(signContract)]
    fn sign_contract(&self, id_contract: u64){
        let caller = self.blockchain().get_caller();
        let contract = self.customer_contracts(&id_contract).get();
        self.require_valid_administrator(contract.id_client, &caller);

        self.customer_contracts(&id_contract).update(|contract| {
            contract.is_signed = true
        });

        self.contracts_client_by_account(&contract.id_client).insert(id_contract);

        self.contract_sign_event(id_contract);
    }

    #[endpoint(addInvoice)]
    fn add_invoice(&self, id_contract: u64, hash: ManagedBuffer, amount: BigUint, due_date: u64){
        let caller = self.blockchain().get_caller();
        let contract = self.customer_contracts(&id_contract).get();
        self.require_valid_administrator(contract.id_supplier, &caller);

        let new_invoice = Invoice {
            hash: hash.clone(),
            amount: amount.clone(),
            identifier: EgldOrEsdtTokenIdentifier::egld(),
            status: Status::PendingValidation,
            due_date: due_date,
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

    #[endpoint(addCompanyAdministrator)]
    fn add_company_administrator(&self, company_id: u64, new_admin: ManagedAddress){
        let caller = self.blockchain().get_caller();
        self.require_valid_administrator(company_id, &caller);

        self.companies(&company_id).update(|val| val.administrators.push(new_admin.clone()));
        self.company_add_admin_event(company_id, new_admin.clone());
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

        invoice.status = Status::Funded;
        self.invoices_by_contract(&id_contract).set(id_invoice as usize, &invoice);
        self.invoice_fund_event(id_contract, id_invoice, self.blockchain().get_block_timestamp());
    }

    #[endpoint(mintWithUnusedLiquidity)]
    fn mint_with_unused_liquidity(&self){
        self.require_caller_is_admin();

        self.mint();
    }

    #[endpoint(enterMarketWithUnusedLiquidity)]
    fn enter_market_with_unused_liquidity(&self){
        self.require_caller_is_admin();

        self.enter_market();
    }

    #[endpoint(exitMarketFarm)]
    fn exit_market_farm(&self){
        self.require_caller_is_admin();

        self.exit_market();
    }

    #[endpoint(withdrawLiquidity)]
    fn withdraw_liquidity(&self){
        self.require_caller_is_admin();

        self.redeem_liquidity();
    }

    #[endpoint(claimFarmingRewards)]
    fn claim_farming_rewards(&self){
        self.require_caller_is_admin();

        self.claim_rewards();
    }

    #[payable("*")]
    #[endpoint(payInvoice)]
    fn pay_invoice(&self, id_contract: u64, id_invoice: u64){
        let caller = self.blockchain().get_caller();
        let contract = self.customer_contracts(&id_contract).get();
        self.require_valid_administrator(contract.id_client, &caller);

        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);

        require!(invoice.status == Status::Funded, INVOICE_NOT_PAYABLE);
        
        
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

        self.pay_invoice_fn(&contract, &mut invoice, id_contract, id_invoice);
    }

    #[endpoint(payInvoiceAuto)]
    fn pay_invoice_auto(&self, id_contract: u64, id_invoice: u64){
        self.require_caller_is_admin();
        
        let contract = self.customer_contracts(&id_contract).get();
        let available_funds = self.funds_by_account(&contract.id_client).get();
        let mut invoice = self.invoices_by_contract(&id_contract).get(id_invoice as usize);
        
        require!(available_funds >= invoice.amount, NOT_ENOUGH_FUNDS);

        self.pay_invoice_fn(&contract, &mut invoice, id_contract, id_invoice);

        self.funds_by_account(&contract.id_client).update(|val| *val -= invoice.amount);
    }

    fn pay_invoice_fn(&self, contract: &CustomerContract, invoice: &mut Invoice<Self::Api>, id_contract: u64, id_invoice: u64){

        let already_paid = BigUint::from(DEFAULT_PERCENT_PAY) * invoice.amount.clone() / BigUint::from(ONE_HUNDRED_PERCENT);
        let remaining_amount = invoice.amount.clone() - already_paid;
        let fees = BigUint::from(DEFAULT_PERCENT_FEE) * invoice.amount.clone() / BigUint::from(ONE_HUNDRED_PERCENT);
        let amount_to_send = remaining_amount - fees;
        let current_timestamp = self.blockchain().get_block_timestamp();

        let company = self.companies(&contract.id_supplier).get();

        self.tx()
            .to(&company.withdraw_address)
            .egld_or_single_esdt(&invoice.identifier, 0, &amount_to_send)
            .transfer_if_not_empty();

        invoice.status = Status::Payed;
        invoice.payed_date = Option::Some(current_timestamp);
        self.invoices_by_contract(&id_contract).set(id_invoice as usize, &invoice);

        self.invoice_pay_event(id_contract, id_invoice, current_timestamp);        
    }

    #[payable("*")]
    #[endpoint(addFunds)]
    fn add_funds(&self) {
        self.require_caller_is_admin();
        self.sc_add_funds_event();
    }

    #[payable("EGLD")]
    #[endpoint(addAccountFunds)]
    fn add_account_funds(&self, id_account: u64) {
        let caller = self.blockchain().get_caller();
        self.require_valid_administrator(id_account, &caller);

        let payment_amount = self.call_value().egld_value().clone_value();
        
        self.funds_by_account(&id_account).update(|val| *val += payment_amount);
        self.company_add_funds_event(id_account);
    }

    fn require_valid_administrator(&self, company_id: u64, caller: &ManagedAddress) {
        let company = self.companies(&company_id).get();

        require!(company.administrators.contains(caller), CALLER_NOT_ADMIN);
    }

    fn get_current_funds(&self, token_identifier: &EgldOrEsdtTokenIdentifier) -> BigUint {
        self.blockchain().get_sc_balance(token_identifier, 0)
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
}
