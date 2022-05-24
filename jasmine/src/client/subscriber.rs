use async_trait::async_trait;
use util::{
    result::{JasmineError, JasmineResult},
    transaction::JasmineMessage,
};
///A trait representing a JasmineSubscriber interface.
#[async_trait]
pub trait JasmineSubscriber {
    ///A function takes in a topic and subscribe the topic.
    async fn subscribe(&self, topic: String) -> JasmineResult<()>;
    ///A function takes in a topic and unsubscribe the topic.
    async fn unsubscribe(&self, topic: String) -> JasmineResult<()>;
}
