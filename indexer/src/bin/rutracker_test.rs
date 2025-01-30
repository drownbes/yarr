use std::env;
use std::sync::Arc;

use indexer::providers::rutracker::client::{RuTrackerClient, RuTrackerConfig};
use indexer::repos::create_sqlite_pool;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env.local")?;

    let pool = create_sqlite_pool("sqlite://db.db").await?;

    let cookie_repo = indexer::repos::cookies_repo::CookiesRepo::new(pool.clone());

    let cookie_repo = Arc::new(Mutex::new(cookie_repo));

    let rt_config = RuTrackerConfig {
        login: env::var("RUTRACKER_LOGIN")?.into(),
        password: env::var("RUTRACKER_PASSWORD")?.into(),
        base_url: env::var("RUTRACKER_BASEURL")?,
        login_path: env::var("RUTRACKER_LOGIN_PATH")?,
        search_path: env::var("RUTRACKER_SEARCH_PATH")?,
        index_path: env::var("RUTRACKER_INDEX_PATH")?,
        topic_path: env::var("RUTRACKER_TOPIC_PATH")?,
        provider_id: "rutracker".to_string(),
    };

    let mut rt = RuTrackerClient::new(rt_config, cookie_repo).await?;

    rt.login().await?;

    if rt.is_session_active().await? {
        println!("Logged in");
    }

    rt.search("Game of thrones".into(), vec![]).await?;

    Ok(())
}
