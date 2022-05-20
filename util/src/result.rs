use std::error::Error;

pub type MQJJResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub enum MQJJError {
    Network(String),
    Unknown(String),
}

pub fn mqjj_error_handler(error: MQJJError) -> MQJJResult<()> {
    todo!();
}

