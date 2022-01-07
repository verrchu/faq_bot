mod scripts;
use scripts::Scripts;

use std::{net::Ipv4Addr, path::PathBuf, sync::Arc};

use redis::{aio::ConnectionManager, AsyncCommands, Client};

#[derive(Clone)]
pub struct Db {
    conn: ConnectionManager,
    scripts: Arc<Scripts>,
}

impl Db {
    pub async fn connect(host: Ipv4Addr, port: u16, scripts_path: PathBuf) -> anyhow::Result<Self> {
        let client = Client::open((host.to_string(), port)).map_err(anyhow::Error::from)?;

        let conn = ConnectionManager::new(client)
            .await
            .map_err(anyhow::Error::from)?;

        let scripts = Scripts::load(scripts_path)?;

        Ok(Self {
            conn,
            scripts: Arc::new(scripts),
        })
    }
}

impl Db {
    pub async fn get_key(&mut self, hash: &str) -> anyhow::Result<PathBuf> {
        self.conn
            .get::<_, String>(hash)
            .await
            .map(PathBuf::from)
            .map_err(anyhow::Error::from)
    }

    pub async fn get_next_keys(&mut self, key: &str) -> anyhow::Result<Vec<PathBuf>> {
        self.conn
            .smembers::<_, Vec<String>>(format!("{}:next", key))
            .await
            .map(|keys| keys.into_iter().map(PathBuf::from).collect())
            .map_err(anyhow::Error::from)
    }

    pub async fn get_segment_name(&mut self, segment: &str, lang: &str) -> anyhow::Result<String> {
        self.conn
            .get(format!("{}:name:{}", segment, lang))
            .await
            .map_err(anyhow::Error::from)
    }
}
