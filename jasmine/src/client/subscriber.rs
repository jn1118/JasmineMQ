use util::{result::{JasmineError, JasmineResult}, transaction::JasmineMessage};

pub trait JasmineSubscriber {
    fn subscribe(topic: String) -> JasmineResult<()>;
    fn unsubscribe(topic: String) -> JasmineResult<()>; 
}