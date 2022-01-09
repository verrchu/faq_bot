mod scripts;
use scripts::Scripts;

use std::{collections::HashMap, net::Ipv4Addr, path::PathBuf, sync::Arc};

use function_name::named;
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
    #[named]
    pub async fn get_key(&mut self, hash: &str) -> anyhow::Result<PathBuf> {
        tracing::debug!(hash, "call {}", function_name!());

        self.conn
            .hget::<_, _, String>("key_hashes", hash)
            .await
            .map(PathBuf::from)
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn is_data_entry(&mut self, key: &str) -> anyhow::Result<bool> {
        tracing::debug!(key, "call {}", function_name!());

        self.conn
            .sismember("data_entries", key)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_next_buttons(
        &mut self,
        key: &str,
        lang: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        tracing::debug!(key, lang, "call {}", function_name!());

        self.conn
            .hgetall(format!("{}:next:{}", key, lang))
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn inc_views(&mut self, key: &str) -> anyhow::Result<u64> {
        tracing::debug!(key, "call {}", function_name!());

        self.conn
            .incr(format!("{}:views", key), 1)
            .await
            .map_err(anyhow::Error::from)
    }
}
