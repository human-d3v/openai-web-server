//region: --- crates
use std::sync::Arc;
use async_openai::{config::OpenAIConfig, Client};
//end region: --- crates

//region: --- modules
use crate::{env_vars::ApiKey, error::OwsError};
//end region: --- modules

//region: --- types
pub type OaClient = Client<OpenAIConfig>;
//end region: --- types

//region: --- functions
pub fn new_oa_client(
    key:ApiKey
) -> Result<Arc<OaClient>, OwsError> { 
    let config = OpenAIConfig::new().with_api_key(key);
    let client: OaClient = Client::with_config(config);
    Ok(Arc::new(client))
}
//end region: --- functions
