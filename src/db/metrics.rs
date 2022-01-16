use crate::metrics::REGISTRY;

use once_cell::sync::Lazy;
use prometheus::{Histogram, HistogramOpts};

pub(super) static POOL_ACCESS: Lazy<Histogram> = Lazy::new(|| {
    Histogram::with_opts(HistogramOpts::new(
        "db:pool_access",
        "retrieve db connection from a pool",
    ))
    .expect("db:pool_access: create metric")
});

pub fn register() {
    REGISTRY
        .register(Box::new(Lazy::force(&POOL_ACCESS).to_owned()))
        .expect("db:pool_access: register metric");
}
