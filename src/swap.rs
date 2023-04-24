use ethers::{
    contract::abigen,
    core::types::Address,
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::Wallet,
    types::U256,
};
use eyre::Result;
use std::{
    io::{self, Write},
    sync::Arc,
};

//needed typings for the swap function input parameter
use crate::get_args::Args;

abigen!(
    IUniswapV2Router02,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)
        function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)
    ]"#;
    IERC20,
    r#"[
        function approve(address spender, uint256 amount) external returns (bool)
        function name() external view returns (string memory)
        function symbol() external view returns (string memory)
        function decimals() external view returns (uint8)
        function allowance(address owner, address spender) external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
    ]"#;
);

#[tokio::main]
pub async fn get_amounts_out(
    args: &Args,
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Result<()> {
    // UniswapV2 Router02 address
    let router_address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    // Instantiate the contract bindings
    let v2_router = IUniswapV2Router02::new(*&router_address, Arc::clone(&client));

    // Extract the token addresses from the command line arguments
    let addr_a = args.token_from.to_string().parse::<Address>()?;
    let addr_b = args.token_to.to_string().parse::<Address>()?;
    let token_path = vec![addr_a, addr_b];

    //Instantiate the token instances
    let addr_a_instance = IERC20::new(addr_a, Arc::clone(&client));
    let addr_b_instance = IERC20::new(addr_b, Arc::clone(&client));

    //get the token symbols and decimals
    let token_a_name = addr_a_instance.symbol().call().await?;
    let token_a_decimals = addr_a_instance.decimals().call().await?;

    let token_b_name = addr_b_instance.symbol().call().await?;
    let token_b_decimals = addr_b_instance.decimals().call().await?;

    // Get the amount of tokens to swap from the user
    let mut input = String::new();
    println!("Enter the amount of ({}) you want to swap:", token_a_name);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let amount_in = input.trim().parse::<u64>().unwrap();

    // Calculate the amount out using the UniswapV2 Router02 contract
    let amounts = v2_router
        .get_amounts_out(
            U256::from(amount_in) * U256::from(10).pow(token_a_decimals.into()),
            token_path.clone()
        )
        .call()
        .await?;

    println!("\n----------------------------------------------------------\n\n");
    println!(
        "Here's an estimated amount of ({}) you'd get after swapping {} {}\n",
        token_b_name, amount_in, token_a_name
    );
    println!(
        "You will receive about: {}{}",
        U256::from(amounts[1]).as_u128() as f64
            / U256::from(10).pow(token_b_decimals.into()).as_u128() as f64,
        token_b_name
    );
    println!("\n\n----------------------------------------------------------\n");

    let mut val: String = String::from("");
    println!("Do you want to continue with the swap? (y/n)");
    io::stdout().flush()?;
    io::stdin().read_line(&mut val)?;
    val = val.trim().to_string();

    if val != "y" {
        println!("Hate to see you go :(");
        std::process::exit(0);
    }

    //MAIN SWAP HAPPENS HERE
    // Approve the UniswapV2 Router02 contract to spend the token
    println!("\n----------------------------------------------------------\n");

    println!(
        "Approving the UniswapV2 Router02 contract to spend {} {} ......\n",
        amount_in, token_a_name
    );
    let _tx = addr_a_instance
        .approve(
            *&router_address,
            U256::from(amount_in) * U256::from(10).pow(token_a_decimals.into()),
        )
        .send()
        .await?
        .await?;

    println!("UniSwapV2 Router02 contract approved.");
    println!("\n----------------------------------------------------------\n\n");

    // Swap the tokens
    println!(
        "Swapping {} {} for {} {}",
        amount_in,
        token_a_name,
        U256::from(amounts[1]).as_u128() as f64
            / U256::from(10).pow(token_b_decimals.into()).as_u128() as f64,
        token_b_name
    );

    let _tx = v2_router
        .swap_exact_tokens_for_tokens(
            U256::from(amount_in) * U256::from(10).pow(token_a_decimals.into()),
            U256::from(amounts[1]),
            token_path.clone(),
            *&client.address(),
            U256::from(10000000000i64),
        )
        .send()
        .await?
        .await?;

    println!("Swap successful.");

    println!("\n----------------------------------------------------------\n\n");

    //check client address balance of token b
    let balance = addr_b_instance
        .balance_of(*&client.address())
        .call()
        .await?;

    println!(
        "Your new balance of {} is: {}",
        token_b_name,
        U256::from(balance).as_u128() as f64
            / U256::from(10).pow(token_b_decimals.into()).as_u128() as f64
    );

    Ok(())
}
