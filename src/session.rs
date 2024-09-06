//region: --- crates
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

//end region: --- crates

//region: --- modules
use crate::oai::thread::ThreadId;
//end region: --- modules

// region: --- types
pub type SessionId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionObj {
    pub session_id: SessionId,
    pub thread_id: ThreadId,

}

#[derive(Debug)]
pub struct ChatSession {
    pub session_map: HashMap<String, String>,
}

impl ChatSession {
    pub fn new() -> Self {
        Self {
            session_map: HashMap::new(),
        }
    }

    pub fn associate_session_with_thread(&mut self, session_id: &SessionId, thread_id: &ThreadId) {
        self.session_map.insert("session_id".to_string(), session_id.to_string());
        self.session_map.insert("thread_id".to_string(), thread_id.to_string());
    }
    
    pub fn get_session_id(&self) -> Option<&str> {
        self.session_map.get("session_id").map(|s| s as &str)
    }
    
    pub fn get_thread_id(&self) -> Option<&str> {
        self.session_map.get("thread_id").map(|s| s as &str)
    }
}
//end region: --- types
