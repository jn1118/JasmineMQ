use util::{
    result::{JasmineError, JasmineResult},
    transaction::JasmineMessage,
};
///A trait representing a JasminePublisher interface.
pub trait JasminePublisher {
    ///A function takes in a topic and message and then publish the topic to correpsponding subscriber.
    fn publish(&self, topic: String, message: JasmineMessage) -> JasmineResult<()>;
}
