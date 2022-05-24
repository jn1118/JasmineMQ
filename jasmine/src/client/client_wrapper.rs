// use util::rpc::client::jasmine_client_server::JasmineClient;
use super::subscriber::JasmineSubscriber;
use crate::client::client::Client;
use crate::client::client::JasmineClient;
use crate::client::publisher::JasminePublisher;
use async_trait::async_trait;
use util::result::JasmineResult;
use util::transaction::JasmineMessage;
pub struct JasmineClientWrapper {
    pub client: Client,
    pub broker_addr: Vec<String>,
}

#[async_trait]
impl JasmineClient for JasmineClientWrapper {
    fn new(&self, broker: Vec<String>) -> JasmineResult<Box<Self>> {
        let result = self.client.new(self.broker_addr);
        match result {
            Ok(value) => {
                return Ok(Box::new(JasmineClientWrapper {
                    client: *value,
                    broker_addr: broker,
                }));
            }
            Err(e) => return Err(e),
        }
    }
    async fn connect(&self) -> JasmineResult<()> {
        let result = self.client.connect().await;
        return result;
    }
    async fn disconnect(&self) -> JasmineResult<()> {
        let result = self.client.disconnect().await;
        return result;
    }
    fn on_message(&self) -> JasmineMessage {
        todo!()
    }
}

#[async_trait]
impl JasmineSubscriber for JasmineClientWrapper {
    async fn subscribe(&self, topic: String) -> JasmineResult<()> {
        let result = self.client.subscribe(topic).await;
        return result;
    }

    async fn unsubscribe(&self, topic: String) -> JasmineResult<()> {
        let result = self.client.unsubscribe(topic).await;
        return result;
    }
}

#[async_trait]
impl JasminePublisher for JasmineClientWrapper {
    async fn publish(&self, topic: String, message: String) -> JasmineResult<()> {
        let result = self.client.publish(topic, message).await;
        return result;
    }
}
