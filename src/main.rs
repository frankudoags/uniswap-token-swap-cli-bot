pub mod get_args;
pub mod get_pool;
mod get_reserves;

fn main() -> eyre::Result<()> {
    let args = get_args::main();
    let pool = get_pool::get_pool(&args)?;
    get_reserves::get_reserves(pool)?;

    Ok(())
}
