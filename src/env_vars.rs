//region: --- crates

use dotenv::dotenv;

use crate::error::Result;

//end region: --- crates
//region: --- types
pub type ApiKey = String;
pub type AsstId = String;
pub type Docker = bool;

pub struct EnvVars {
    pub api_key: ApiKey,
    pub asst_id: AsstId,
    pub docker: Docker,
}
//end region: --- types


//region: --- main
pub fn retrieve_env_vars() -> Result<EnvVars> {
    if dotenv().is_err() {
        eprintln!("no .env file found");
    } else {
        println!("found .env file")
    }
    let _ = dotenv::from_filename(".env");
    let api_key:ApiKey = std::env::var("API_KEY")?;
    let asst_id:AsstId = std::env::var("ASSISTANT_ID")?;
    let docker: Docker = std::env::var("DOCKER")?.parse()?;

    let env_vars = EnvVars { api_key, asst_id, docker };
    Ok(env_vars)
}
//end region: --- main
