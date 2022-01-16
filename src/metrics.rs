use http::StatusCode;
use once_cell::sync::Lazy;
use prometheus::{Registry, TextEncoder};

pub static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

pub fn register() {
    crate::db::metrics::register();
}

pub async fn gather() -> Result<String, StatusCode> {
    match TextEncoder::new().encode_to_string(&REGISTRY.gather()) {
        Ok(metrics) => Ok(metrics),
        Err(err) => {
            tracing::error!("failed to gather metrics: {:?}", err);

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
