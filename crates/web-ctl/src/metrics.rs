use axum::response::IntoResponse;
use prometheus::{Encoder, TextEncoder};

pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metrics_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metrics_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
