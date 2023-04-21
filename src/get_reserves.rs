use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Http, Provider},
};
use eyre::Result;
use std::sync::Arc;

use crate::get_args::Args;

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
        function token0() external view returns (address)
        function token1() external view returns (address)
    ]"#,
);

abigen!(
    IERC20,
    r#"[
        function name() external view returns (string memory)
    ]"#,
);

pub struct TokensAndReserves {
    pub token0: Address,
    pub token1: Address,
    pub token0_name: String,
    pub token1_name: String,
    pub reserve0: u128,
    pub reserve1: u128,
}

#[tokio::main]
pub async fn get_reserves(
    pool: Address,
    args: &Args,
    client: Arc<Provider<Http>>,
) -> Result<TokensAndReserves> {
    // instantiate the pair contract bindings
    let pair = IUniswapV2Pair::new(pool, Arc::clone(&client));
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;

    // get the reserves
    // get the token addresses
    let token0 = pair.token_0().call().await?;
    let token1 = pair.token_1().call().await?;
    // get the token names from the token addresses
    let token0_name = IERC20::new(token0, Arc::clone(&client))
        .name()
        .call()
        .await?;
    let token1_name = IERC20::new(token1, Arc::clone(&client))
        .name()
        .call()
        .await?;

    // calculate the mid price, no idea if this is correct, but hey it works
    // copied from ethers-rs examples: ethers-rs/examples/queries/examples/uniswapv2_pair.rs
    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;

    // use the args to determine which token is the input token
    // and which token is the output token
    // if the input token is token0, then the output token is token1
    // and vice versa

    let input_token = args.token_from.to_string().parse::<Address>()?;
    let output_token = args.token_to.to_string().parse::<Address>()?;

    if input_token == token0 {
        println!("You are swapping {} for {}", token0_name, token1_name);
        println!(
            "Price of {} in {}: {} \n\n\n",
            token0_name, token1_name, mid_price
        );
    } else if input_token == token1 {
        println!("You are swapping {} for {}", token1_name, token0_name);
        println!(
            "Price of {} in {}: {} \n\n\n",
            token1_name,
            token0_name,
            1.0 / mid_price
        );
    } else {
        std::process::exit(0);
    }

    Ok(TokensAndReserves {
        token0: input_token,
        token1: output_token,
        token0_name,
        token1_name,
        reserve0: reserve0.into(),
        reserve1: reserve1.into(),
    })
}
