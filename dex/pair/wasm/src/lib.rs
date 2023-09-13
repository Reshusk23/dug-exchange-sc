// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           54
// Async Callback (empty):               1
// Total number of exported functions:  56

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    pair
    (
        addInitialLiquidity
        addLiquidity
        removeLiquidity
        removeLiquidityAndBuyBackAndBurnToken
        swapNoFeeAndForward
        swapTokensFixedInput
        swapTokensFixedOutput
        setLpTokenIdentifier
        getTokensForGivenPosition
        getReservesAndTotalSupply
        getAmountOut
        getAmountIn
        getEquivalent
        getFeeState
        whitelist
        removeWhitelist
        addTrustedSwapPair
        removeTrustedSwapPair
        setupFeesCollector
        setFeeOn
        getFeeDestinations
        getTrustedSwapPairs
        getWhitelistedManagedAddresses
        getFeesCollectorAddress
        getFeesCollectorCutPercentage
        setStateActiveNoSwaps
        setFeePercents
        getLpTokenIdentifier
        getTotalFeePercent
        getSpecialFee
        getRouterManagedAddress
        getFirstTokenId
        getSecondTokenId
        getTotalSupply
        getInitialLiquidtyAdder
        getReserve
        getSafePriceCurrentIndex
        updateAndGetTokensForGivenPositionWithSafePrice
        updateAndGetSafePrice
        setLockingDeadlineEpoch
        setLockingScAddress
        setUnlockEpoch
        getLockingScAddress
        getUnlockEpoch
        getLockingDeadlineEpoch
        addAdmin
        removeAdmin
        updateOwnerOrAdmin
        getPermissions
        addToPauseWhitelist
        removeFromPauseWhitelist
        pause
        resume
        getState
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
