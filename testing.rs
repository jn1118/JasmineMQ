client = Client::new()......

fn return_value(topic, message) {
    return message;
}

client.on_connect(return_value)