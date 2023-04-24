use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
    utils::Anvil,
};
use eyre::Result;
use std::sync::Arc;

pub mod get_args;
pub mod get_pool;
mod swap;

fn main() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.llamarpc.com").spawn();
    let provider = Provider::<Http>::try_from(anvil.endpoint())?;
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);
    

    // Get the arguments from the command line
    let args = get_args::main();

    // Get the pool address from the arguments or throw an error
    let _pool = get_pool::get_pool(&args, client.clone())?;

    // get the amount to swap and return a quote, swap the tokens if the user confirms
    swap::get_amounts_out(&args, client.clone())?;

    Ok(())
}

