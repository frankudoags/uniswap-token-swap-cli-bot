## A Command Line Tool for swapping tokens on Uniswap Protocol
This is a command line tool for swapping tokens on Uniswap Protocol. It is written in Rust and uses the [ethers-rs](https://github.com/gakonst/ethers-rs) library. It is a work in progress and is not yet ready for production use.

### Usage
This tool is not yet published on crates.io. To use it, clone the repository and run the following command:
```
cargo run -q
```
Follow the prompts to enter the token addresses and amounts you want to swap. The tool will then print the transaction hash and the URL to view the transaction on Etherscan. These features are not complete yet as this tool is still in development.
Cheers!


You'd need to add an env file with the following variables:
```
PRIVATE_KEY= YOUR_PRIVATE_KEY
```
The tool will use this private key to sign the transaction. You can get a private key from Metamask or any other Ethereum wallet.
NOT READY YET AGAIN, STILL IN DEVELOPMENT, DO NOT USE