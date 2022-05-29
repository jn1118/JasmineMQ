use redis::{Connection, Commands};
use util::result::JasmineResult;
extern crate redis;

/// Install Redis: sudo apt install redis-server
/// Start Redis Server: redis-server


pub struct Storage {
    conn: Connection,
    key_id: usize, 
}

impl Storage {

    /// Create a new storage client
    pub fn new() -> JasmineResult<Self> {
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let conn = client.get_connection()?;

        Ok(Storage{conn, key_id: 0})
    }

    /// Store a vector of u8, return the key
    pub fn set(&mut self, bytes: Vec<u8>) -> JasmineResult<String> {

        let key = self.key_id.to_string();
        self.key_id += 1;
        self.conn.set(key.clone(), bytes)?;
        return Ok(key);
        
    }

    /// Get the data with the key
    pub fn get(&mut self, key: &str) -> JasmineResult<Vec<u8>> {
        let bin : Vec<u8> = self.conn.get(key)?;
        Ok(bin)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_set_and_get() {
        let data = vec![0x41u8, 0x41u8, 0x42u8];
        let mut storage = Storage::new().unwrap();
        let key = storage.set(data.clone()).unwrap();
        let bin_data = storage.get(&key).unwrap();
        assert_eq!(bin_data, data);
    }

}