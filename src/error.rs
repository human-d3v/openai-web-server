use std::fmt::Display;

//region: --- types
pub type CustomError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, CustomError>;
//end region: --- types


#[derive(Debug)]
pub enum OwsError {
    OpenaiError(async_openai::error::OpenAIError),
    EnvError(String)
}

impl Display for OwsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           Self::OpenaiError(e)=> write!(f, "{}", e),
           Self::EnvError(msg) => write!(f, "{}", msg), 
        }
    }
}

impl From<async_openai::error::OpenAIError> for OwsError {
    fn from(value: async_openai::error::OpenAIError) -> Self {
        Self::OpenaiError(value)
    }
}

impl From<dotenv::Error> for OwsError {
    fn from(value: dotenv::Error) -> Self {
        Self::EnvError(value)
    }
}
