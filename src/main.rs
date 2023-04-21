use std::sync::Arc;

use ethers::providers::{Http, Provider};

pub mod get_args;
pub mod get_pool;
mod get_reserves;
mod swap;
pub use get_reserves::TokensAndReserves;

fn main() -> eyre::Result<()> {
    // let RPC_URL: &str = "https://eth.llamarpc.com";
    // Create a new provider from anvil forking mainnet
    let client = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    // Wrap the provider in an Arc so that it can be shared
    let client = Arc::new(client);

    // Get the arguments from the command line
    let args = get_args::main();

    // Get the pool address from the arguments or throw an error
    let pool = get_pool::get_pool(&args, client.clone())?;

    // Get the reserves and token addresses and price or throw an error
    let details: TokensAndReserves = get_reserves::get_reserves(pool, &args, client.clone())?;

    // get the amount to swap and return a quote, swap the tokens if the user confirms
    swap::get_amounts_out_and_swap(&details, client.clone())?;
    
    Ok(())
}
