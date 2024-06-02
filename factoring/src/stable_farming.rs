multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StableFarmingModule : 
    crate::storage::Storage {

    fn mint_enter(&self, token_identifier: EgldOrEsdtTokenIdentifier, amount: BigUint, mm_sc_address: ManagedAddress) {

        let payment = EgldOrEsdtTokenPayment::new(token_identifier, 0u64, amount);

        self.hatom_proxy(mm_sc_address)
            .mint_and_enter_market(OptionalValue::<ManagedAddress>::None)
            .with_egld_or_single_esdt_transfer(payment)
            .async_call_and_exit();
    }

    fn claim_rewards(&self) {

        let controller_sc_address = self.hatom_controller_address().get();

        let money_markets = ManagedVec::<Self::Api, ManagedAddress<Self::Api>>::new();
        let accounts = ManagedVec::<Self::Api, ManagedAddress<Self::Api>>::new();

        self.hatom_proxy(controller_sc_address)
            .claim_rewards(false, true, true, money_markets, accounts)
            .async_call_and_exit();
    }

    fn exit_market(&self, mm_sc_address: ManagedAddress, amount: BigUint) {

        let controller_sc_address = self.hatom_controller_address().get();

        self.hatom_proxy(controller_sc_address)
            .exit_market(mm_sc_address, OptionalValue::Some(amount))
            .async_call_and_exit();
    }

    fn redeem_liquidity(&self, token_identifier: EgldOrEsdtTokenIdentifier, amount: BigUint, mm_sc_address: ManagedAddress) {

        let payment = EgldOrEsdtTokenPayment::new(token_identifier, 0u64, amount);

        self.hatom_proxy(mm_sc_address)
            .redeem(OptionalValue::<BigUint>::None)
            .with_egld_or_single_esdt_transfer(payment)
            .async_call_and_exit();
    }

    #[proxy]
    fn hatom_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> crate::hatom_proxy::Proxy<Self::Api>;

}