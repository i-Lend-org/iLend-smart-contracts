#[cfg(test)]
mod tests {
    use crate::utils::success_deposit_of_diff_token_with_prices;
    use cosmwasm_std::Addr;
    use cw_multi_test::Executor;
    use lending::msg::ExecuteMsg;

    #[test]
    #[should_panic(expected = "Allowed for Admin Only")]
    fn test_fail_set_reserve_configuration_if_caller_is_not_owner() {
        const PERCENT_DECIMALS: u32 = 5;
        const LTV_ETH: u128 = 92 * 10u128.pow(PERCENT_DECIMALS); // 92%
        const LIQUIDATION_THRESHOLD_ETH: u128 = 98 * 10u128.pow(PERCENT_DECIMALS); // 98%

        let (mut app, addr) = success_deposit_of_diff_token_with_prices();

        app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::SetReserveConfiguration {
                denom: "eth".to_string(),
                loan_to_value_ratio: LTV_ETH,
                liquidation_threshold: LIQUIDATION_THRESHOLD_ETH,
            },
            &[],
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "Token Not Supported")]
    fn test_fail_set_reserve_configuration_if_token_is_not_supported() {
        const PERCENT_DECIMALS: u32 = 5;
        const LTV_ETH: u128 = 92 * 10u128.pow(PERCENT_DECIMALS); // 92%
        const LIQUIDATION_THRESHOLD_ETH: u128 = 98 * 10u128.pow(PERCENT_DECIMALS); // 98%

        let (mut app, addr) = success_deposit_of_diff_token_with_prices();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::SetReserveConfiguration {
                denom: "usdt".to_string(),
                loan_to_value_ratio: LTV_ETH,
                liquidation_threshold: LIQUIDATION_THRESHOLD_ETH,
            },
            &[],
        )
        .unwrap();
    }
}
