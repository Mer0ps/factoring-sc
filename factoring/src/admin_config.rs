multiversx_sc::imports!();

use crate::errors::*;

#[multiversx_sc::module]
pub trait AdminConfigModule :
    super::events::EventsModule
    + crate::stable_farming::StableFarmingModule
    + crate::storage::Storage
 {

    #[payable("*")]
    #[endpoint(addProcolFunds)]
    fn add_protocol_funds(&self) {
        self.require_caller_is_admin();
        let (token_identifier, _, payment_amount) = self.call_value().egld_or_single_esdt().into_tuple();
        self.allowed_tokens().require_whitelisted(&token_identifier);

        self.protocol_funds(&token_identifier).update(|val| *val += payment_amount);
        
        self.sc_add_funds_event();
    }

    #[endpoint(setHatomControllerAddress)]
    fn set_hatom_controller_address(&self, sc_address: ManagedAddress) {
        self.require_caller_is_admin();

        self.hatom_controller_address().set(&sc_address);
    }

    #[endpoint(removeProcolFunds)]
    fn remove_protocol_funds(&self, token_identifier: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        self.require_caller_is_admin();
        self.allowed_tokens().require_whitelisted(&token_identifier);

        let available_funds = self.protocol_funds(&token_identifier).get();

        require!(available_funds >= amount, NOT_ENOUGH_FUNDS);



        self.protocol_funds(&token_identifier).update(|val| *val -= amount);
        
        self.sc_add_funds_event();
    }

    #[endpoint(mintWithUnusedLiquidity)]
    fn mint_with_unused_liquidity(&self, token_identifier: EgldOrEsdtTokenIdentifier, amount: BigUint, mm_sc_address: ManagedAddress){
        self.require_caller_is_admin();
        
        let token_amount_available = self.blockchain().get_sc_balance(
            &token_identifier,
            0u64,
        );

        require!(token_amount_available >= amount, NOT_ENOUGH_FUNDS);

        self.mint_enter(token_identifier, amount, mm_sc_address);
    }

    #[endpoint(exitMarketFarm)]
    fn exit_market_farm(&self, mm_sc_address: ManagedAddress, amount: BigUint){
        self.require_caller_is_admin();

        self.exit_market(mm_sc_address, amount);
    }

    #[endpoint(withdrawLiquidity)]
    fn withdraw_liquidity(&self, token_identifier: EgldOrEsdtTokenIdentifier, amount: BigUint, mm_sc_address: ManagedAddress){
        self.require_caller_is_admin();

        let token_amount_available = self.blockchain().get_sc_balance(
            &token_identifier,
            0u64,
        );

        require!(token_amount_available >= amount, NOT_ENOUGH_FUNDS);

        self.redeem_liquidity(token_identifier, amount, mm_sc_address);
    }

    #[endpoint(claimFarmingRewards)]
    fn claim_farming_rewards(&self){
        self.require_caller_is_admin();

        self.claim_rewards();
    }

    #[endpoint(addUserToAdminList)]
    fn add_user_to_admin_list(&self, address: ManagedAddress) {
        self.require_caller_is_admin();
        self.admin_whitelist().add(&address);
        self.sc_add_admin_event(address);
    }

    #[endpoint(removeUserFromAdminList)]
    fn remove_user_from_admin_list(&self, address: ManagedAddress) {
        self.require_caller_is_admin();
        self.admin_whitelist().remove(&address);
        
        self.sc_remove_admin_event(address);
    }

    #[endpoint(addEuriborRate)]
    fn add_euribor_rate(&self, timestamp: u64, rate: u32) {
        self.require_caller_is_admin();

        if self.euribor_rate().is_empty() {
            
            self.euribor_rate().set((rate, timestamp));

        }else{
            
            let (_, old_timestamp) = self.euribor_rate().get();
            require!(timestamp > old_timestamp, TIMESTAMP_MUST_BE_HIGHER);
            self.euribor_rate().set((rate, timestamp));
        }
    }

    #[endpoint(addAllowedToken)]
    fn add_allowed_tokens(&self, token: EgldOrEsdtTokenIdentifier) {
        self.require_caller_is_admin();

        self.allowed_tokens().add(&token);
        self.sc_add_token_event(token);
    }

    #[endpoint(removeAllowedToken)]
    fn remove_allowed_tokens(&self, token: EgldOrEsdtTokenIdentifier) {
        self.require_caller_is_admin();

        self.allowed_tokens().remove(&token);
        
        self.sc_remove_token_event(token);
    }

    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        let sc_owner = self.blockchain().get_owner_address();
        if caller == sc_owner {
            return;
        }

        self.admin_whitelist().require_whitelisted(&caller);
    }

    fn get_current_funds(&self, token_identifier: &EgldOrEsdtTokenIdentifier) -> BigUint {
        self.blockchain().get_sc_balance(token_identifier, 0)
    }

    #[storage_mapper("adminWhitelist")]
    fn admin_whitelist(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    #[storage_mapper("euriborRate")]
    fn euribor_rate(&self) -> SingleValueMapper<(u32, u64)>;

    #[storage_mapper("allowedTokens")]
    fn allowed_tokens(&self) -> WhitelistMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getProtocolFunds)]
    #[storage_mapper("protocol_funds")]
    fn protocol_funds(&self, identifier: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;
}