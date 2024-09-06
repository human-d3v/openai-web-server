//region: --- crates
use serde::{Deserialize, Serialize};
//end region: --- crates

//region: --- modules
//end region: --- modules

//region: --- types
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMsg {
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgFromPage {
    pub message: Vec<InputMsg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgToPage {
    pub message: String,
}
//end region: --- types

//region: --- message constructor
//end region: --- message constructor
