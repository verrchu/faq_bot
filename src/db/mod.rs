mod scripts;
use scripts::Scripts;

use std::{collections::BTreeMap, net::Ipv4Addr, path::PathBuf, sync::Arc};

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
    ) -> anyhow::Result<BTreeMap<String, String>> {
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

    #[named]
    pub async fn is_feedback_process_active(&mut self, user_id: i64) -> anyhow::Result<bool> {
        tracing::debug!(user_id, "call {}", function_name!());

        self.conn
            .hexists("feedback", user_id)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn get_feedback_message_id(&mut self, user_id: i64) -> anyhow::Result<Option<i32>> {
        tracing::debug!(user_id, "call {}", function_name!());

        self.conn
            .hget("feedback", user_id)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn begin_feedback_process(
        &mut self,
        user_id: i64,
        message_id: i32,
    ) -> anyhow::Result<bool> {
        tracing::debug!(user_id, message_id, "call {}", function_name!());

        self.conn
            .hset_nx("feedback", user_id, message_id)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn end_feedback_process(&mut self, user_id: i64) -> anyhow::Result<bool> {
        tracing::debug!(user_id, "call {}", function_name!());

        self.conn
            .hdel("feedback", user_id)
            .await
            .map_err(anyhow::Error::from)
    }
}
