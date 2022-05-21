use util::result::JasmineResult;
extern crate redis;


pub struct Storage {

}

impl Storage {

    /// Store a vector of u8, return the key
    pub fn set(bytes: Vec<u8>) -> JasmineResult<String> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        let key = "key".to_string();
        return Ok(key);
    }

    /// Get the data with the key
    pub fn get(key: &str) -> JasmineResult<Vec<u8>> {
        todo!();
    }
}