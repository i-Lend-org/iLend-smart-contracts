#[cfg(test)]
mod tests {
    use crate::utils::success_collateral_margin_setup_with_deposit;
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::Executor;

    use margin_positions::msg::{
        ExecuteMsg as ExecuteMsgMarginPositions, QueryMsg as QueryMsgMarginPositions,
    };

    #[test]
    fn test_success_redeem_one_token_whole_amount() {
        const TOKENS_DECIMALS: u32 = 18;
        const INIT_USER_BALANCE: u128 = 1000 * 10u128.pow(TOKENS_DECIMALS);
        const RESERVE_AMOUNT: u128 = 1000 * 10u128.pow(TOKENS_DECIMALS);
        const FIRST_DEPOSIT_AMOUNT_ETH: u128 = 200 * 10u128.pow(TOKENS_DECIMALS);
        const SECOND_DEPOSIT_AMOUNT_ETH: u128 = 300 * 10u128.pow(TOKENS_DECIMALS);

        let (mut app, margin_positions_addr, collateral_contract_addr) =
            success_collateral_margin_setup_with_deposit();

        app.execute_contract(
            Addr::unchecked("user"),
            margin_positions_addr.clone(),
            &ExecuteMsgMarginPositions::Redeem {
                denom: "eth".to_string(),
                amount: Uint128::from(FIRST_DEPOSIT_AMOUNT_ETH + SECOND_DEPOSIT_AMOUNT_ETH),
            },
            &[],
        )
        .unwrap();

        let user_deposited_balance_after_first_withdrawal: Uint128 = app
            .wrap()
            .query_wasm_smart(
                margin_positions_addr.clone(),
                &QueryMsgMarginPositions::GetDeposit {
                    address: "user".to_string(),
                    denom: "eth".to_string(),
                },
            )
            .unwrap();

        assert_eq!(user_deposited_balance_after_first_withdrawal.u128(), 0);

        assert_eq!(
            app.wrap()
                .query_balance("user", "eth")
                .unwrap()
                .amount
                .u128(),
            INIT_USER_BALANCE
        );

        assert_eq!(
            app.wrap()
                .query_balance(collateral_contract_addr.clone(), "eth")
                .unwrap()
                .amount
                .u128(),
            RESERVE_AMOUNT
        );
    }
}