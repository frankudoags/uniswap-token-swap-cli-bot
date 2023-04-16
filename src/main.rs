pub mod get_args;
pub mod get_pool;

fn main() -> eyre::Result<()> {
    let args = get_args::main();
    get_pool::get_pool(&args)?;

    Ok(())
}
