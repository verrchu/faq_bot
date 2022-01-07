use std::net::Ipv4Addr;

use redis::{Client, aio::ConnectionManager};

#[derive(Clone)]
pub struct Db {
    conn: ConnectionManager,
}

impl Db {
    pub async fn connect(host: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        let client = Client::open((host.to_string(), port))
            .map_err(anyhow::Error::from)?;

        let conn = ConnectionManager::new(client)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(Self { conn })
    }
}
