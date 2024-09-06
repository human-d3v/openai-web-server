//region: --- crates
use async_openai::types::{MessageAttachment, MessageAttachmentTool};
//end region: --- crates

//region: --- modules
use crate::error::Result;
use super::client::OaClient;
//end region: --- modules

//region: --- types
pub type VecStoreId = String;
pub type FileId = String;
//end region: --- types

//region: --- constants
pub const DEFAULT_QUERY: &[(&str, &str)] = &[("limit","100")];
//end region: --- constants

//region: --- vector storage handling
pub async fn load_vec_store(
    client:&OaClient,
) -> Result<VecStoreId> {
    let vec_stores = client.vector_stores().list(DEFAULT_QUERY).await?;
    let vec_store:VecStoreId = vec_stores.data[0].id.clone();
    Ok(vec_store)
}

pub async fn loac_vec_store_files(
    client:&OaClient,
    vec_store_id:&VecStoreId
) -> Result<Vec<FileId>> {
    let files = client.vector_stores()
        .files(vec_store_id)
        .list(DEFAULT_QUERY).await?;
    let id_vec: Vec<FileId> = files.data.iter()
        .map(|f| f.id.clone()).collect();
    Ok(id_vec)
}

pub async fn build_attachments_obj(
    file_vec:Vec<FileId>
) -> Result<Vec<MessageAttachment>> {
    let mut attachments: Vec<MessageAttachment> = [].to_vec();
    for file in file_vec {
        attachments.push(
            MessageAttachment {
                file_id: file,
                tools: vec![MessageAttachmentTool::FileSearch],
            }
        )
    }
    Ok(attachments)
}

//end region: --- vector storage handling
