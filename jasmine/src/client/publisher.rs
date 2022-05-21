use util::{result::{JasmineError, JasmineResult}, transaction::JasmineMessage};

pub trait JasminePublisher {
    fn publish(&self, topic: String, message: JasmineMessage) -> JasmineResult<()>;
}





