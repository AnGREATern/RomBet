use std::env;
use std::fs::OpenOptions;
use tracing::subscriber;
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt};

pub fn init_default_logger() {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(env::var("ROM_BET_LOG_NAME").expect("ROM_BET_LOG_NAME didn't setup"))
        .unwrap();
    let subscriber = Registry::default().with(fmt::layer().json().with_writer(log_file));
    
    subscriber::set_global_default(subscriber).expect("Logger already exists");
}
