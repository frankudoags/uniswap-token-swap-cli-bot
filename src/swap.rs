use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Http, Provider},
    types::U256,
};
use eyre::Result;
use std::{sync::Arc, str::FromStr};

//needed typings for the swap function input parameter
use crate::get_reserves::TokensAndReserves;



abigen!(
    IUniswapV2Router02,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)
    ]"#,
);

/// This function will instantiate and call the UniswapV2 Router02 contract to get the amounts out
/// for a given amount in and swap the tokens.
/// 
/// # Errors
/// 
/// This function will return an error if :
/// - The UniswapV2 Router02 address is invalid
/// - The amount in is invalid
/// - The contract call fails
/// - The contract call returns an error

#[tokio::main]
pub async fn get_amounts_out_and_swap(details: &TokensAndReserves, client: Arc<Provider<Http>>) -> Result<()> {
    println!("SWAPPING TOKENS");
    // UniswapV2 Router02 address
    let address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    // Instantiate the contract bindings
    let v2_router = IUniswapV2Router02::new(address, Arc::clone(&client));

    // Extract the token addresses from the command line arguments
    let addr_a = details.token0;
    let addr_b = details.token1;
    let path = vec![addr_a, addr_b];
    println!("Path: {:?}", path);

    let mut input: String = String::new();
    println!("Enter the amount of {} to swap: ", details.token0_name);
    // Read the input from the user
    std::io::stdin().read_line(&mut input)?;

    // Parse the input as a U256
    let amount_in = U256::from_str(&input.trim_end())?;
    println!("Amount in: {}", amount_in);

    // Call the contract method to get the amounts out
    let amounts = v2_router.get_amounts_out(amount_in, path).call().await?;
    println!("Amounts: {:?}", amounts);
    Ok(())
}
