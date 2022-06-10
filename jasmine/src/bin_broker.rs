use tokio::sync::mpsc::Sender;
use util::{config::BROKER_ADDRS, result::JasmineResult};
pub fn main() -> JasmineResult<()> {
    let mut brokers = Vec::new();
    for i in BROKER_ADDRS {
        brokers.push(i.to_string())
    }

    spawn_broker(brokers);
    dbg!("idididid");
    // env_logger::builder()
    //     .default_format()
    //     .filter_level(log_level)
    //     .init();
    // let config = Arc::new(Config::read(Some(&cfg))?);

    // println!("{:?}", config);
    // let (tx, rdy) = mpsc::channel();

    // let mut handles = vec![];
    // let it = match t {
    //     ProcessType::Back => &config.backs,
    //     ProcessType::Keep => &config.keepers,
    // };
    // for (i, srv) in it.iter().enumerate() {
    //     if addr::check(srv)? {
    //         handles.push(tokio::spawn(run_srv(
    //             t.clone(),
    //             i,
    //             config.clone(),
    //             Some(tx.clone()),
    //         )));
    //     }
    // }
    // let proc_name = match t {
    //     ProcessType::Back => "backend",
    //     ProcessType::Keep => "keeper",
    // };
    // if handles.is_empty() {
    //     warn!("no {}s found for this host", proc_name);
    //     return Ok(());
    // }
    // info!("Waiting for ready signal from {}...", proc_name);
    // match rdy.recv_timeout(Duration::from_secs(recv_timeout)) {
    //     Ok(msg) => match msg {
    //         true => info!("{}s should be ready to serve.", proc_name),
    //         false => {
    //             error!("{}s failed to start successfully", proc_name);
    //             process::exit(1);
    //         }
    //     },
    //     Err(_) => {
    //         error!("timed out waiting for {}s to start", proc_name);
    //         process::exit(1);
    //     }
    // }
    // for h in handles {
    //     match join!(h) {
    //         (Ok(_),) => (),
    //         (Err(e),) => {
    //             warn!("A {} failed to join: {}", proc_name, e);
    //         }
    //     };
    // }
    Ok(())
}

fn spawn_broker(
    brokers: Vec<String>,
) -> (
    Vec<tokio::task::JoinHandle<JasmineResult<()>>>,
    Vec<Sender<()>>,
) {
    let mut handles = vec![];
    let mut brokers_shutdown = vec![];

    for i in 0..3 {
        let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
        let l = tokio::spawn(jasmine::library::initialize_broker(
            brokers.clone(),
            i,
            shut_rx,
        ));
        brokers_shutdown.push(shut_tx);
        handles.push(l);
    }
    return (handles, brokers_shutdown);
}
