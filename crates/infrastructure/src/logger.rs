use std::fs::OpenOptions;
use std::{env, str::FromStr};
use tracing::{Level, level_filters::LevelFilter, subscriber};
use tracing_subscriber::{Layer, Registry, fmt, layer::SubscriberExt};

pub fn init_default_logger() {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(env::var("ROM_BET_LOG_NAME").expect("ROM_BET_LOG_NAME didn't setup"))
        .unwrap();

    let level =
        Level::from_str(&env::var("ROM_BET_LOG_LEVEL").expect("ROM_BET_LOG_LEVEL didn't setup"))
            .unwrap_or(Level::DEBUG);
    let level_filter = LevelFilter::from_level(level);
    let subscriber = Registry::default().with(
        fmt::layer()
            .json()
            .with_writer(log_file)
            .with_filter(level_filter),
    );

    subscriber::set_global_default(subscriber).expect("Logger already exists");
}
