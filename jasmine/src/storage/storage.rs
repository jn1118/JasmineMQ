use util::result::JasmineResult;

pub struct Storage {

}

impl Storage {

    /// Store a vector of u8, return the key
    pub fn set(bytes: Vec<u8>) -> JasmineResult<String> {
        let key = "key".to_string();
        return Ok(key);
    }

    /// Get the data with the key
    pub fn get(key: &str) -> JasmineResult<Vec<u8>> {
        todo!();
    }
}