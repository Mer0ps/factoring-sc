use multiversx_sc::hex_literal::hex;
multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StableFarmingModule {

    fn mint(&self) {
        let usdc_identifier = TokenIdentifier::from_esdt_bytes(b"USDC-350c4e");

        let usdc_balance = self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(usdc_identifier.clone()),
            0u64,
        );

        //hatom mm sc on devnet
        let mm_sc_address = ManagedAddress::from(hex!(
            "0000000000000000050089e0c6a6b8e7bce45cefe166089c14e23e0cb8256509"
        ));

        let payment = EsdtTokenPayment::new(usdc_identifier, 0u64, usdc_balance);

        self.hatom_proxy(mm_sc_address)
            .mint()
            .with_esdt_transfer(payment)
            .async_call()
            .call_and_exit();
    }

    fn enter_market(&self) {
        let husdc_identifier = TokenIdentifier::from_esdt_bytes(b"HUSDC-9b1b64");

        let husdc_balance = self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(husdc_identifier.clone()),
            0u64,
        );

        //hatom controller sc on devnet
        let controller_sc_address = ManagedAddress::from(hex!(
            "00000000000000000500d6074059eb7a8d8a06d07e7bd3afebf9370626e36509"
        ));

        let payment = EsdtTokenPayment::new(husdc_identifier, 0u64, husdc_balance);

        self.hatom_proxy(controller_sc_address)
            .enter_markets(OptionalValue::<BigUint>::None)
            .with_esdt_transfer(payment)
            .async_call()
            .call_and_exit();
    }

    fn claim_rewards(&self) {

        //hatom controller sc on devnet
        let controller_sc_address = ManagedAddress::from(hex!(
            "00000000000000000500d6074059eb7a8d8a06d07e7bd3afebf9370626e36509"
        ));

        let money_markets = ManagedVec::<Self::Api, ManagedAddress<Self::Api>>::new();
        let accounts = ManagedVec::<Self::Api, ManagedAddress<Self::Api>>::new();

        self.hatom_proxy(controller_sc_address)
            .claim_rewards(false, true, true, money_markets, accounts)
            .async_call()
            .call_and_exit();
    }

    fn exit_market(&self) {

        //hatom controller sc on devnet
        let controller_sc_address = ManagedAddress::from(hex!(
            "00000000000000000500d6074059eb7a8d8a06d07e7bd3afebf9370626e36509"
        ));

        //hatom mm sc on devnet
        let mm_sc_address = ManagedAddress::from(hex!(
            "0000000000000000050089e0c6a6b8e7bce45cefe166089c14e23e0cb8256509"
        ));

        self.hatom_proxy(controller_sc_address)
            .exit_market(mm_sc_address, OptionalValue::<BigUint>::None)
            .async_call()
            .call_and_exit();
    }

    fn redeem_liquidity(&self) {
        let husdc_identifier = TokenIdentifier::from_esdt_bytes(b"HUSDC-9b1b64");

        let husdc_balance = self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(husdc_identifier.clone()),
            0u64,
        );

        //hatom mm sc on devnet
        let mm_sc_address = ManagedAddress::from(hex!(
            "0000000000000000050089e0c6a6b8e7bce45cefe166089c14e23e0cb8256509"
        ));

        let payment = EsdtTokenPayment::new(husdc_identifier, 0u64, husdc_balance);

        self.hatom_proxy(mm_sc_address)
            .redeem(OptionalValue::<BigUint>::None)
            .with_esdt_transfer(payment)
            .async_call()
            .call_and_exit();
    }

    #[proxy]
    fn hatom_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> crate::hatom_proxy::Proxy<Self::Api>;

}