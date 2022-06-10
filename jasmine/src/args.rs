use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
// #[clap(client)]
pub struct JasmineArgs {
    #[clap(subcommand)]
    pub command: UserCommand,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum UserCommand {
    StartBroker(StartBroker),
    Client(CreateClient),
    Publish(PublishMessage),
    Subscribe(SubscribeTopic),
    Unubscribe(UnsubscribeTopic),
    Retrieve(ReceiveMessage),
}

#[derive(Debug, Args, PartialEq)]
pub struct StartBroker {
    pub num: usize,
}

#[derive(Debug, Args, PartialEq)]
pub struct CreateClient {
    pub name: String,
}
#[derive(Debug, Args, PartialEq)]
//publish a b hihih false
pub struct PublishMessage {
    pub name: String,
    pub topic: String,
    pub message: String,
    pub is_consistent: usize,
}

#[derive(Debug, Args, PartialEq)]
pub struct SubscribeTopic {
    pub name: String,
    pub topic: String,
}
#[derive(Debug, Args, PartialEq)]
pub struct UnsubscribeTopic {
    pub name: String,
    pub topic: String,
}

#[derive(Debug, Args, PartialEq)]
pub struct ReceiveMessage {
    pub name: String,
    pub message: String,
    pub is_consistent: usize,
}
