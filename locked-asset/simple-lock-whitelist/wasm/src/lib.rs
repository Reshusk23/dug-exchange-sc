// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           24
// Async Callback:                       1
// Total number of exported functions:  26

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    simple_lock_whitelist
    (
        setTransferRoleLockedToken
        setTransferRoleProxyLpToken
        setTransferRoleProxyFarmToken
        setLockedToken
        lockTokens
        unlockTokens
        getTokenWhitelist
        issueLockedToken
        getLockedTokenId
        issueLpProxyToken
        addLpToWhitelist
        removeLpFromWhitelist
        addLiquidityLockedToken
        removeLiquidityLockedToken
        getKnownLiquidityPools
        getLpProxyTokenId
        issueFarmProxyToken
        addFarmToWhitelist
        removeFarmFromWhitelist
        enterFarmLockedToken
        exitFarmLockedToken
        farmClaimRewardsLockedToken
        getKnownFarms
        getFarmProxyTokenId
        callBack
    )
}
