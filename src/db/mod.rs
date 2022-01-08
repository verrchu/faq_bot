mod scripts;
use scripts::Scripts;

use crate::Lang;

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
        tracing::debug!("getting key (hash: {})", hash);

        self.conn
            .hget::<_, _, String>("key_hashes", hash)
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

    pub async fn get_segment_name(&mut self, segment: &str, lang: Lang) -> anyhow::Result<String> {
        self.conn
            .get(format!("{}:name:{}", segment, lang))
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_segment_names<I, S>(
        &mut self,
        segments: I,
        lang: Lang,
    ) -> anyhow::Result<Vec<String>>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let keys = segments
            .into_iter()
            .map(|segment| format!("{}:name:{}", segment.as_ref(), lang))
            .collect::<Vec<_>>();

        redis::cmd("MGET")
            .arg(keys)
            .query_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_key_icons<I, S>(&mut self, keys: I) -> anyhow::Result<Vec<Option<String>>>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let keys = keys
            .into_iter()
            .map(|key| format!("{}:icon", key.as_ref()))
            .collect::<Vec<_>>();

        redis::cmd("MGET")
            .arg(keys)
            .query_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn is_data_entry(&mut self, key: &str) -> anyhow::Result<bool> {
        self.conn
            .sismember("data_entries", key)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_key_data(&mut self, key: &str, lang: Lang) -> anyhow::Result<String> {
        self.conn
            .get(format!("{}:data:{}", key, lang))
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn get_key_created(&mut self, key: &str) -> anyhow::Result<u64> {
        self.conn
            .get(format!("{}:created", key))
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn inc_views(&mut self, key: &str) -> anyhow::Result<u64> {
        self.conn
            .incr(format!("{}:views", key), 1)
            .await
            .map_err(anyhow::Error::from)
    }
}
