// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           31
// Async Callback (empty):               1
// Total number of exported functions:  34

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    factoring
    (
        init => init
        upgrade => upgrade
        addCompany => add_company
        createFactoringContract => create_factoring_contract
        signContract => sign_contract
        addInvoice => add_invoice
        calculateFinancingFees => calculate_financing_fees
        calculateCommission => calculate_commission
        confirmInvoice => confirm_invoice
        addCompanyAdministrator => add_company_administrator
        fundInvoice => fund_invoice
        payInvoice => pay_invoice
        payInvoiceAuto => pay_invoice_auto
        fundRemainingAmount => fund_remaining_amount
        addAccountFunds => add_account_funds
        calculateReliabilityScore => calculate_reliability_score
        addFunds => add_funds
        mintWithUnusedLiquidity => mint_with_unused_liquidity
        enterMarketWithUnusedLiquidity => enter_market_with_unused_liquidity
        exitMarketFarm => exit_market_farm
        withdrawLiquidity => withdraw_liquidity
        claimFarmingRewards => claim_farming_rewards
        addUserToAdminList => add_user_to_admin_list
        removeUserFromAdminList => remove_user_from_admin_list
        addEuriborRate => add_euribor_rate
        addAllowedToken => add_allowed_tokens
        removeAllowedToken => remove_allowed_tokens
        getCompany => companies
        getCustomerContract => customer_contracts
        getInvoicesByContract => invoices_by_contract
        getContractsClientByAccount => contracts_client_by_account
        getAvailableAssetByAccountAndIdentifier => assets_by_account_and_identifier
        getFundsByAccount => funds_by_account
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
