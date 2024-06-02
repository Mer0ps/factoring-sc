multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait HatomProxyModule {

    //Method from money-market.abi
    #[payable("*")]
    #[endpoint(mintAndEnterMarket)]
    fn mint_and_enter_market(&self, opt_account : OptionalValue<ManagedAddress>) -> EsdtTokenPayment;

    //Method from money-market.abi
    #[payable("*")]
    #[endpoint(redeem)]
    fn redeem(&self, underlying_amount : OptionalValue<BigUint>) -> MultiValue2<EgldOrEsdtTokenPayment, EsdtTokenPayment>;

    //Method from controller.abi
    #[endpoint(exitMarket)]
    fn exit_market(&self, money_market: ManagedAddress, opt_tokens : OptionalValue<BigUint>) -> EsdtTokenPayment;

    //Method from controller.abi
    #[endpoint(claimRewards)]
    fn claim_rewards(&self, boost: bool, supply: bool, borrow: bool, money_markets: ManagedVec<ManagedAddress>, accounts: ManagedVec<ManagedAddress>);
}