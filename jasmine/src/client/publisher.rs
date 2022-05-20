use util::{result::{JasmineError, JasmineResult}, transaction::JasmineMessage};

pub trait JasminePublisher {
    fn publish(topic: String, message: JasmineMessage) -> JasmineResult<()>;
}





