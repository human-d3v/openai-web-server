//region: --- crates
#[macro_use] extern crate rocket;
use rocket::{serde::json::Json, tokio::sync::Mutex, Config, State};
use rocket_cors::CorsOptions;
use std::{collections::HashMap, sync::Arc};
use async_openai::types::{AssistantObject, ThreadObject};
//end region: --- crates

//region: --- modules
pub mod error;
use oai::{asst::load_asst, client::{new_oa_client, OaClient}, msg::{MsgFromPage, MsgToPage}, thread::{fetch_thread, handle_chat, spawn_thread, ThreadId}, vec_store::{load_vec_store, FileId, VecStoreId}};
mod oai;
pub mod env_vars;
use env_vars::retrieve_env_vars;
mod session;
use session::{ChatSession, SessionId, SessionObj};
//end region: --- modules

// region: --- types
struct AppState {
    sessions: ChatSession,
    asst: AssistantObject,
    client: Arc<OaClient>,
    vec_store_id: VecStoreId,
}

//end region: --- types

// region: --- routes
#[options("/chat")]
pub fn preflight() -> rocket::http::Status { // bypass cors block
    rocket::http::Status::Ok
}

#[get("/session/<session_id>")]
async fn new_session (
    session_id: SessionId,
    state: &State<Arc<Mutex<AppState>>>
)-> Result<Json<SessionObj>,String> {
    let mut app_state = state.inner().lock().await;
    let thread_id: ThreadId = spawn_thread(
        &app_state.client,
        &app_state.vec_store_id
    ).await.expect("error spawning thread");
    app_state.sessions.associate_session_with_thread(&session_id, &thread_id);
    Ok(Json(
            SessionObj { 
                session_id: app_state.sessions
                    .get_session_id()
                    .unwrap()
                    .to_string() ,
                thread_id: app_state.sessions
                    .get_thread_id()
                    .unwrap()
                    .to_string() 
            }
        )
    )
}

#[get("/thread")]
async fn thread_info(
    state: &State<Arc<Mutex<AppState>>>, 
) -> Result<Json<ThreadObject>, String> {
    let app_state = state.inner().lock().await;
    let thread = fetch_thread(
        &app_state.sessions.get_thread_id().unwrap().to_string(),
        &app_state.client
    ).await.expect("unable to find thread");
    Ok(Json(thread))
}

#[post("/chat", format="json", data="<frontend_msg>")]
async fn chat(
    frontend_msg: Json<MsgFromPage>,
    state: &State<AppState>,
) -> Result<Json<MsgToPage>, String> {
    let thread_id = state.sessions.get_thread_id().unwrap().to_string();
    let msg = handle_chat(
        &state.client, 
        &state.asst.id, 
        &thread_id, 
        &frontend_msg.message[0].input
    ).await.expect("error handling chat");
    let msg_html = markdown::to_html(&msg);
    let msg_to_render = MsgToPage {message: msg_html};
    Ok(Json(msg_to_render))
}
// end region: --- routes

// region: launch 
#[launch]
async fn rocket() -> _ {
    //session
    let session_map:ChatSession = ChatSession::new();
     
    //get env vars
    let env_vars = retrieve_env_vars()
        .expect("env vars not found");

    //get client
    let client = new_oa_client(env_vars.api_key)
        .expect("no client found");

    //get assistant
    let asst = load_asst(&client, &env_vars.asst_id).await
        .expect("no assistant found");

    //access vector storage id
    let vec_store_id: VecStoreId = load_vec_store(&client).await
        .expect("no vector store found");

    let app_state = AppState {
        sessions: session_map,
        client,
        asst,
        vec_store_id,
    };

    let cors_opts = CorsOptions {
        ..Default::default()
    };

    let config = Config {
        address: "0.0.0.0".parse().expect("address error"),
        port: 8080,
        ..Default::default()
    };

    rocket::custom(config)
        .attach(cors_opts.to_cors().expect("cors error"))
        .manage(app_state)
        .mount("/", routes![preflight, new_session, chat, thread_info])
}
// end region: launch
