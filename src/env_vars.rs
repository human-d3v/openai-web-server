//region: --- crates

use dotenv::dotenv;

use crate::error::OwsError;


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
pub fn retrieve_env_vars() -> Result<EnvVars, Box<dyn std::error::Error> {
    dotenv::from_filename(".env")
        .map_err(|e| OwsError::EnvError(e.to_string()))?;
    let api_key:ApiKey = std::env::var("API_KEY")?;
    let asst_id:AsstId = std::env::var("ASSISTANT_ID")?;
    let docker: Docker = std::env::var("DOCKER")?.parse()?;

    let env_vars = EnvVars { api_key, asst_id, docker };
    Ok(env_vars)
}
//end region: --- main
