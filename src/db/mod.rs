mod scripts;
use scripts::Scripts;

use crate::Lang;

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
    pub async fn get_next_keys(&mut self, key: &str) -> anyhow::Result<Vec<PathBuf>> {
        tracing::debug!(key, "call {}", function_name!());

        self.conn
            .smembers::<_, Vec<String>>(format!("{}:next", key))
            .await
            .map(|keys| keys.into_iter().map(PathBuf::from).collect())
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_segment_names<I, S>(
        &mut self,
        segments: I,
        lang: &Lang,
    ) -> anyhow::Result<Vec<String>>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let lang = lang.as_str();

        tracing::debug!(lang, "call {}", function_name!());

        let segments = segments
            .into_iter()
            .map(|segment| format!("{}:name:{}", segment.as_ref(), lang))
            .collect::<Vec<_>>();

        redis::cmd("MGET")
            .arg(segments)
            .query_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_key_icons<I, S>(&mut self, keys: I) -> anyhow::Result<Vec<Option<String>>>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        tracing::debug!("call {}", function_name!());

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

    #[named]
    pub async fn is_data_entry(&mut self, key: &str) -> anyhow::Result<bool> {
        tracing::debug!(key, "call {}", function_name!());

        self.conn
            .sismember("data_entries", key)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_key_data(&mut self, key: &str, lang: &Lang) -> anyhow::Result<String> {
        let lang = lang.as_str();

        tracing::debug!(key, lang, "call {}", function_name!());

        self.conn
            .get(format!("{}:data:{}", key, lang))
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_key_created(&mut self, key: &str) -> anyhow::Result<u64> {
        tracing::debug!(key, "call {}", function_name!());

        self.conn
            .get(format!("{}:created", key))
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_next_buttons(
        &mut self,
        key: &str,
        lang: &Lang,
    ) -> anyhow::Result<HashMap<String, String>> {
        let lang = lang.as_str();

        tracing::debug!(key, lang, "call {}", function_name!());

        self.conn
            .hgetall(format!("{}:next:{}", key, lang))
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn inc_views(&mut self, key: &str) -> anyhow::Result<u64> {
        tracing::info!(key, "call {}", function_name!());

        self.conn
            .incr(format!("{}:views", key), 1)
            .await
            .map_err(anyhow::Error::from)
    }
}
