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
    IUniswapV2Factory,
    r#"[
        function getPair(address tokenA, address tokenB) external view returns (address pair)
    ]"#,
);

#[tokio::main]
pub async fn get_pool(args: &Args) -> Result<Address> {
    // Create a new provider pointing to the specified RPC URL
    let client = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    // Wrap the provider in an Arc so that it can be shared
    let client = Arc::new(client);

    // UniswapV2 Factory address
    let address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;
    // Instantiate the contract bindings
    let factory = IUniswapV2Factory::new(address, Arc::clone(&client));

    // Extract the token addresses from the command line arguments
    let addr_a = args.token_from.to_string().parse::<Address>()?;
    let addr_b = args.token_to.to_string().parse::<Address>()?;

    // Call the contract method to get the pair address
    let pair = factory.get_pair(addr_a, addr_b).call().await?;

    // check if the pair is zero
    // if it is, then no pool exists for the given tokens, so we exit
    // if it isn't, then a pool exists for the given tokens
    // and we can print the address of the pool
    if pair == Address::zero() {
        println!("Sorry,no uniswapv2 pool was found for the tokens you're trying to swap");
        std::process::exit(0);
    } else {
        println!("Pool found: {}", pair);
        Ok(pair)
    }
}

// WETH ADDRESS: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
// USDT ADDRESS: 0xdAC17F958D2ee523a2206206994597C13D831ec7
// WETH/USDT PAIR: 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852