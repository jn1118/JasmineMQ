use std::error::Error;

pub type JasmineResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub enum JasmineError {
    Network(String),
    Unknown(String),
}
