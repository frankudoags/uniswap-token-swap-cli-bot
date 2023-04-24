## A Command Line Tool for swapping tokens on Uniswap Protocol
This is a command line tool for swapping tokens on Uniswap Protocol. It is written in Rust and uses the [ethers-rs](https://github.com/gakonst/ethers-rs) library. It is a work in progress and is not yet ready for production use.

### Usage
This tool is not yet published on crates.io. To use it, clone the repository and run the following command:
```
cargo run -q
```
Follow the prompts to enter the token addresses and amounts you want to swap. The tool will then print the transaction hash and the URL to view the transaction on Etherscan. These features are not complete yet as this tool is still in development.
Cheers!

As of now, this project works with an Anvil mainnet fork, but it will be updated to work with the mainnet later on.

Here are some input examples:
```
Enter the address of the token you want to swap from: 
0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2


Enter the address of the token you want to swap to: 0xdAC17F958D2ee523a2206206994597C13D831ec7
```

These prompts are for WETH and USDT respectively. The tool will then prompt you to enter the amount of WETH you want to swap for USDT. Here is an example:
```
Enter the amount of WETH you want to swap for USDT: 0.1
```

WETH is the only token that can be swapped from at the moment. This will be updated later on.
```