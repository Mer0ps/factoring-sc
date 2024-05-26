multiversx_sc::imports!();

use crate::{storage, errors::*, company::*, contract::CustomerContract};

pub const DEFAULT_SCORE: u8 = 100;

#[multiversx_sc::module]
pub trait AccountManagerModule :
    super::events::EventsModule
    + crate::admin_config::AdminConfigModule
    + crate::stable_farming::StableFarmingModule
    + storage::Storage
{
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

    #[endpoint(addCompanyAdministrator)]
    fn add_company_administrator(&self, company_id: u64, new_admin: ManagedAddress){
        let caller = self.blockchain().get_caller();
        self.require_valid_administrator(company_id, &caller);

        self.companies(&company_id).update(|val| val.administrators.push(new_admin.clone()));
        self.company_add_admin_event(company_id, new_admin.clone());
    }

    #[payable("*")]
    #[endpoint(addAccountFunds)]
    fn add_account_funds(&self, id_account: u64) {
        let caller = self.blockchain().get_caller();

        let (token_identifier, _, payment_amount) = self.call_value().egld_or_single_esdt().into_tuple();

        self.allowed_tokens().require_whitelisted(&token_identifier);
        self.require_valid_administrator(id_account, &caller);
        
        self.funds_by_account(&id_account, &token_identifier).update(|val| *val += payment_amount);
        self.company_add_funds_event(id_account);
    }

    #[endpoint(removeAccountFunds)]
    fn remove_account_funds(&self, id_account: u64, token_identifier: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        let caller = self.blockchain().get_caller();

        self.allowed_tokens().require_whitelisted(&token_identifier);
        self.require_valid_administrator(id_account, &caller);

        let available_funds = self.funds_by_account(&id_account, &token_identifier).get();

        require!(available_funds >= amount, NOT_ENOUGH_FUNDS);
        
        self.funds_by_account(&id_account, &token_identifier).update(|val| *val -= amount);
        self.company_remove_funds_event(id_account);
    }

    fn require_valid_administrator(&self, company_id: u64, caller: &ManagedAddress) {
        let company = self.companies(&company_id).get();

        require!(company.administrators.contains(caller), CALLER_NOT_ADMIN);
    }


}