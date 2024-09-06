
use std::time::Duration;

//region: --- crates
use async_openai::{types::{CreateAssistantToolFileSearchResources, CreateAssistantToolResources, CreateMessageRequest, CreateMessageRequestContent, CreateRunRequest, CreateThreadRequest, MessageContent, MessageObject, MessageRole, RunStatus, ThreadObject}, Threads};
use rocket::tokio::time::sleep;
//end region: --- crates

//region: --- modules
use crate::{env_vars::AsstId, error::Result};
use super::{client::OaClient, vec_store::VecStoreId};
//end region: --- modules

//region: --- types
pub type ThreadId = String;
//end region: --- types

//region: --- constants
pub const POLLING_DURATION_MS:u64 = 500;
//end region: --- constants

//region: --- threads
pub async fn spawn_thread(
    client: &OaClient,
    vec_store_id: &VecStoreId
) -> Result<ThreadId> {
    let request = CreateThreadRequest {
        tool_resources: Some(CreateAssistantToolResources {
            file_search: Some(CreateAssistantToolFileSearchResources { 
                vector_store_ids: Some(vec![vec_store_id.to_string()]),
                vector_stores: None}),
            code_interpreter: None,
        }),
        ..Default::default()
    };
    let thread = Threads::new(client).create(request).await?;
    Ok(thread.id)
}

pub async fn fetch_thread(
    thread_id: &ThreadId,
    client: &OaClient
) -> Result<ThreadObject>{
    let thread = client.threads().retrieve(thread_id).await?;
    Ok(thread)
}

pub async fn handle_chat(
    client: &OaClient,
    asst_id: &AsstId,
    thread_id: &ThreadId,
    frontend_msg: &str,
) -> Result<String> {
    //build msg obj
    let msg = CreateMessageRequest {
        role: MessageRole::User,
        content: CreateMessageRequestContent::Content(frontend_msg.to_string()),
        ..Default::default()
    };

    //Openai has three steps:
    // 1 --- attach message to thread
    let _msg_obj = client.threads().messages(thread_id).create(msg).await?;
    // 2 --- create run request
    let run_request = CreateRunRequest {
        assistant_id: asst_id.to_string(),
        ..Default::default()
    };
    // 3 --- send the run 
    let run = client.threads().runs(thread_id).create(run_request).await?;
    // 4 --- loop to get the result
    loop {
        let run = client.threads().runs(thread_id).retrieve(&run.id).await?;
        match run.status {
            RunStatus::Completed => {
                return Ok(
                    get_first_thread_msg_content(client, thread_id).await?
                );
            },
            RunStatus::Queued | RunStatus::InProgress => (),
            other => return Err(
                format!("Unexpected run status: {other:?}").into()
            ),
        };
        sleep(Duration::from_millis(POLLING_DURATION_MS)).await
    }
}

pub async fn get_first_thread_msg_content(
    client: &OaClient,
    thread_id: &ThreadId
) -> Result<String>{
    static QUERY: [(&str, &str); 1] = [("limit", "1")];
    let msgs = client.threads().messages(thread_id).list(&QUERY).await?;

    let msg = msgs
        .data
        .into_iter()
        .next()
        .ok_or_else(|| "No messages found in thread".to_string())?;
    let msg_txt = get_txt_content(msg)?;
    Ok(msg_txt)
}

pub fn get_txt_content(
    msg: MessageObject
) -> Result<String> {
    let msg_content = msg
        .content
        .into_iter()
        .next()
        .ok_or_else(|| "No message content found".to_string())?;

    let txt = match msg_content {
        MessageContent::Text(text) => text.text.value,
        MessageContent::ImageUrl(_) => {
            return Err("Message image not supported".into())
        }
        other => return Err(
            format!("Unexpected message content: {other:?}")
            .into()
        )
    };
    Ok(txt)
}
//end region: --- threads
