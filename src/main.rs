//region: --- crates
//end region: --- crates

//region: --- modules
pub mod error;
use error::Result;
mod oai;
pub mod env_vars;
//end region: --- modules

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
