use reaper_eth_engine_macros::action_impl;
use reaper_eth_engine_pricing::Protocol;
use reaper_eth_engine_types::{
    normalized_actions::NormalizedMint, structured_trace::CallInfo, ToScaledRational,
};

action_impl!(
    Protocol::CurveV1MetapoolImpl,
    crate::CurveV1MetapoolImpl::add_liquidity_0Call,
    Mint,
    [..AddLiquidity],
    logs: true,
    |
    info: CallInfo,
    log: CurveV1MetapoolImplAdd_liquidity_0CallLogs,
    db_tx: &DB|{
        let log = log.add_liquidity_field?;

        let details = db_tx.get_protocol_details(info.from_address)?;
        let token_addrs = vec![details.token0, details.curve_lp_token.ok_or(eyre::eyre!("Expected 'curve_lp_token', found 'None'"))?];
        let protocol = details.protocol;

        let amounts = log.token_amounts;
        let (tokens, token_amts): (Vec<_>, Vec<_>) = token_addrs.into_iter()
.enumerate().map(|(i, t)|
        {
            let token = db_tx.try_fetch_token_info(t)?;
            let decimals = token.decimals;
            Ok((token, amounts[i].to_scaled_rational(decimals)))
        }
        ).collect::<eyre::Result<Vec<_>>>()?.into_iter().unzip();


        Ok(NormalizedMint {
            protocol,
            trace_index: info.trace_idx,
            pool: info.from_address,
            from: info.msg_sender,
            recipient: info.msg_sender,
            token: tokens,
            amount: token_amts,
        })
    }
);

// could not find any V1 metapools calling this
action_impl!(
    Protocol::CurveV1MetapoolImpl,
    crate::CurveV1MetapoolImpl::add_liquidity_1Call,
    Mint,
    [..AddLiquidity],
    logs: true,
    |
    info: CallInfo,
    log: CurveV1MetapoolImplAdd_liquidity_1CallLogs,
    db_tx: &DB|{
        let log = log.add_liquidity_field?;

        let details = db_tx.get_protocol_details(info.from_address)?;
        let token_addrs = vec![details.token0, details.curve_lp_token.ok_or(eyre::eyre!("Expected 'curve_lp_token', found 'None'"))?];
        let protocol = details.protocol;

        let amounts = log.token_amounts;
        let (tokens, token_amts): (Vec<_>, Vec<_>) = token_addrs.into_iter()
.enumerate().map(|(i, t)|
        {
            let token = db_tx.try_fetch_token_info(t)?;
            let decimals = token.decimals;
            Ok((token, amounts[i].to_scaled_rational(decimals)))
        }
        ).collect::<eyre::Result<Vec<_>>>()?.into_iter().unzip();


        Ok(NormalizedMint {
            protocol,
            trace_index: info.trace_idx,
            pool: info.from_address,
            from: info.msg_sender,
            recipient: info.msg_sender,
            token: tokens,
            amount: token_amts,
        })
    }
);

#[cfg(test)]
mod tests {

    use alloy_primitives::{hex, Address, B256, U256};
    use reaper_eth_engine_classifier::test_utils::ClassifierTestUtils;
    use reaper_eth_engine_types::{
        db::token_info::{TokenInfo, TokenInfoWithAddress},
        normalized_actions::Action,
        TreeSearchBuilder,
    };

    use super::*;

    #[reaper_eth_engine_macros::test]
    async fn test_curve_v1_metapool_add_liquidity0() {
        let classifier_utils = ClassifierTestUtils::new().await;
        classifier_utils.ensure_protocol(
            Protocol::CurveV1MetaPool,
            Address::new(hex!("A77d09743F77052950C4eb4e6547E9665299BecD")),
            Address::new(hex!("6967299e9F3d5312740Aa61dEe6E9ea658958e31")),
            Some(Address::new(hex!("6B175474E89094C44Da98b954EedeAC495271d0F"))),
            Some(Address::new(hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"))),
            Some(Address::new(hex!("dAC17F958D2ee523a2206206994597C13D831ec7"))),
            None,
            Some(Address::new(hex!("6c3F90f043a72FA612cbac8115EE7e52BDe6E490"))),
        );

        let mint =
            B256::from(hex!("00723506614af2c7a56057b0bd70c263198019e58aac8c10b337d2391996ea0f"));

        let token0 = TokenInfoWithAddress {
            address: Address::new(hex!("6967299e9F3d5312740Aa61dEe6E9ea658958e31")),
            inner:   TokenInfo { decimals: 18, symbol: "T".to_string() },
        };

        let token1 = TokenInfoWithAddress {
            address: Address::new(hex!("6c3f90f043a72fa612cbac8115ee7e52bde6e490")),
            inner:   TokenInfo { decimals: 18, symbol: "3Crv".to_string() },
        };

        classifier_utils.ensure_token(token0.clone());
        classifier_utils.ensure_token(token1.clone());

        let eq_action = Action::Mint(NormalizedMint {
            protocol:    Protocol::CurveV1MetaPool,
            trace_index: 1,
            from:        Address::new(hex!("1a734e9bDa6893915928eE8edBA75cA17536d385")),
            recipient:   Address::new(hex!("1a734e9bDa6893915928eE8edBA75cA17536d385")),
            pool:        Address::new(hex!("A77d09743F77052950C4eb4e6547E9665299BecD")),
            token:       vec![token0, token1],
            amount:      vec![
                U256::from(1000000000000000000000_u128).to_scaled_rational(18),
                U256::from(1000000000000000000000_u128).to_scaled_rational(18),
            ],
        });

        classifier_utils
            .contains_action(
                mint,
                0,
                eq_action,
                TreeSearchBuilder::default().with_action(Action::is_mint),
            )
            .await
            .unwrap();
    }
}