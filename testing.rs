client = Client::new()......

fn return_value(topic, message) {
    return topic;
}

client.on_connect(return_value)