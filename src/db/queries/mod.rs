pub mod feedback;
pub mod grid;
pub mod utils;

use std::time::Instant;

use super::{metrics, Db};

use anyhow::Context;
use r2d2::PooledConnection;
use redis::cluster::ClusterClient;

fn get_connection(db: &Db) -> anyhow::Result<PooledConnection<ClusterClient>> {
    let start = Instant::now();
    let conn = db.pool.get().context("db::get_connection");
    metrics::POOL_ACCESS.observe(start.elapsed().as_secs_f64());
    conn
}
