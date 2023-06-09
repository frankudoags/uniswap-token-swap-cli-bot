use ethers::{
    contract::abigen,
    core::types::Address,
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::Wallet,
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
pub async fn get_pool(
    args: &Args,
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Result<Address> {
    // UniswapV2 Factory address, parsed into an Address type(H160)
    let address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;
    // Instantiate the contract bindings
    let factory = IUniswapV2Factory::new(address, Arc::clone(&client));

    // Extract the token addresses from the command line arguments
    let addr_a = args.token_from.to_string().parse::<Address>()?;
    let addr_b = args.token_to.to_string().parse::<Address>()?;

    // Call the contract method to get the pair address
    let pair = factory.get_pair(addr_a, addr_b).call().await?;

    // check if the pair is zero, if it is, no pool was found
    if pair == Address::zero() {
        println!("Sorry,no uniswapv2 pool was found for the tokens you're trying to swap");
        std::process::exit(0);
    } else {
        println!("\n----------------------------------------------------------");
        println!("UniswapV2 Pool found:\n {}", pair.to_string());
        println!("----------------------------------------------------------\n");
        Ok(pair)
    }
}

// WETH ADDRESS: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
// USDT ADDRESS: 0xdAC17F958D2ee523a2206206994597C13D831ec7
// WETH/USDT PAIR: 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852
