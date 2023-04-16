use ethers::{
    contract::abigen,
    core::{
        types::Address,
        utils::parse_ether,
    },
    
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

#[tokio::main]
pub async fn get_reserves(pool: Address) -> Result<()> {
    // Create a new provider pointing to the specified RPC URL
    let client = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    // Wrap the provider in an Arc so that it can be shared
    let client = Arc::new(client);

    // instantiate the pair contract bindings
    let pair = IUniswapV2Pair::new(pool, Arc::clone(&client));
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
    
    
    // get the reserves
    // get the token addresses
    let token0 = pair.token_0().call().await?;
    let token1 = pair.token_1().call().await?;
    // get the token names from the token addresses
    let token0_name = IERC20::new(token0, Arc::clone(&client)).name().call().await?;
    let token1_name = IERC20::new(token1, Arc::clone(&client)).name().call().await?;

    // print the reserves with the token names
    println!("Reserves for {} and {}: {} and {}", token0_name, token1_name, reserve0, reserve1);
    // calculate the mid price, no idea if this is correct, copied from ethers-rs examples: ethers-rs/examples/queries/examples/uniswapv2_pair.rs
    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    println!("Price of {} in {}: {}", token0_name, token1_name, mid_price);
    Ok(())
}
