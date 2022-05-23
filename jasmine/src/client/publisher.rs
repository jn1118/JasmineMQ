use async_trait::async_trait;
use util::{
    result::{JasmineError, JasmineResult},
    transaction::JasmineMessage,
};
///A trait representing a JasminePublisher interface.
#[async_trait]
pub trait JasminePublisher {
    ///A function takes in a topic and message and then publish the topic to correpsponding subscriber.
    async fn publish(&self, topic: String, message: String) -> JasmineResult<()>;
}
