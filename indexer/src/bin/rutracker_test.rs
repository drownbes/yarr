use std::env;
use std::sync::Arc;
use std::time::Duration;

use indexer::logs::init_tracing;
use indexer::providers::rutracker::client::{RuTrackerClient, RuTrackerConfig};
use indexer::repos::create_sqlite_pool;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    dotenvy::from_filename(".env.local")?;

    let pool = create_sqlite_pool("sqlite://db.db").await?;

    let cookie_repo = indexer::repos::cookies_repo::CookiesRepo::new(pool.clone());

    let cookie_repo = Arc::new(Mutex::new(cookie_repo));

    let rt_config = RuTrackerConfig {
        login: env::var("RUTRACKER_LOGIN")?.into(),
        password: env::var("RUTRACKER_PASSWORD")?.into(),
        base_url: env::var("RUTRACKER_BASEURL")?,
        provider_id: "rutracker".to_string(),
        timeout: Duration::from_secs(30),
    };

    let mut rt = RuTrackerClient::new(rt_config, cookie_repo).await?;

    rt.login().await?;

    if rt.is_session_active().await? {
        println!("Logged in");
    }

    let res = rt.search("Game of thrones".into(), vec![]).await?;

    dbg!(res);

    Ok(())
}
