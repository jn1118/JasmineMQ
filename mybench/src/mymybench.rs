use jasmine::{broker::broker::Broker, client::client::Client};

fn genreate_addrs(base_addr: String, start_port: u64, num: u64) -> Vec<String> {
    let mut addrs = Vec::<String>::new();
    for i in 0..num {
        let mut temp_addr = base_addr.clone();
        temp_addr.push_str(&((start_port + i).to_string()));
        addrs.push(temp_addr);
    }

    return addrs;
}

fn spawn_broker(num: u64) -> (Vec<String>, Vec<Broker>) {
    let mut addrs = Vec::<String>::new();
    let base_addr = "127.0.0.1:".to_string();
    for i in 0..num {
        let addr = base_addr.clone().push_str(&i.to_string());
        addrs.push(addr);
    }

    return wioefjwioejfwoief;
}

fn spawn_client(num: u64) -> Vec<Client> {}
