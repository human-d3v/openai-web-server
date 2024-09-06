//region: --- crates
use async_openai::types::{
    AssistantObject, 
    AssistantTools, 
    ModifyAssistantRequest
};
//end region: --- crates

//region: --- modules
use crate::{env_vars::AsstId, error::Result};
use super::client::OaClient;
//end region: --- modules

//region: --- Assistant
pub async fn load_asst(
    client: &OaClient,
    asst_id: &AsstId
) -> Result<AssistantObject> {
    let asst = client.assistants().retrieve(asst_id).await?;
    Ok(asst)
}

pub async fn update_asst(
    client: &OaClient, 
    asst: &AsstId
) -> Result<()> {
    let req: ModifyAssistantRequest = ModifyAssistantRequest {
        tools: Some(vec![AssistantTools::FileSearch(Default::default())]),
        ..Default::default()
    };
    client.assistants().update(asst, req).await?;
    Ok(())
}
//end region: --- Assistant
