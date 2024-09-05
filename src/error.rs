//region: --- types
pub type CustomError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, CustomError>;
//end region: --- types
