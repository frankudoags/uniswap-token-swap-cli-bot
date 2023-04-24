#[allow(dead_code)]
use std::io::stdin;

#[derive(Debug)]
pub struct Args {
    pub token_from: String,
    pub token_to: String,
}

pub(crate) fn main() -> Args {
    println!("----------------------------------------------------------");
    println!("UNISWAP TOKEN SWAP CLI");
    println!("This is a CLI tool to swap tokens on Uniswap");
    println!("----------------------------------------------------------\n");

    println!("Enter the token you want to swap from: ");
    let mut token_from = String::new();
    stdin()
        .read_line(&mut token_from)
        .expect("Failed to read line");

    println!("\nEnter the token you want to swap to: ");
    let mut token_to = String::new();
    stdin()
        .read_line(&mut token_to)
        .expect("Failed to read line");

    let args = Args {
        token_from: token_from.trim_end().to_string(),
        token_to: token_to.trim_end().to_string(),
    };

    args
}
