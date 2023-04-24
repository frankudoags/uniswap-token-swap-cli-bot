use ethers::{
    prelude::{abigen, SignerMiddleware, k256::ecdsa::SigningKey},
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
    types::Address,
    utils::{parse_ether, Anvil},
};
use eyre::Result;
use std::sync::Arc;

pub mod get_args;
pub mod get_pool;
mod swap;

abigen!(
    IWETH,
    r#"[
        function deposit() external payable
        function withdraw(uint wad) external
        function balanceOf(address guy) external view returns (uint wad)
    ]"#,
);
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

    // Convert some ETH to WETH
    swap_to_weth(client.clone())?;

    // get the amount to swap and return a quote, swap the tokens if the user confirms
    swap::get_amounts_out(&args, client.clone())?;

    Ok(())
}

#[tokio::main]
async fn swap_to_weth(client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) -> Result<()> {
    //Convert some ETH to WETH
    let weth_address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>()?;
    let weth = IWETH::new(weth_address, Arc::clone(&client));
    //convert 100ETH to WETH
    weth.deposit()
        .value(parse_ether(100usize)?) // 100 ETH
        .send()
        .await?;

    // Get the WETH balance
    let balance = weth.balance_of(client.address()).call().await?;
    println!(
        "WETH balance: {} WETH",
        balance / 1_000_000_000_000_000_000u128
    );
    Ok(())
}
