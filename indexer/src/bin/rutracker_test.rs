use std::env;

use indexer::providers::rutracker::{RuTrackerConfig, RuTrackerProvider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env.local")?;

    let rt_config = RuTrackerConfig {
        login: env::var("RUTRACKER_LOGIN")?.into(),
        password: env::var("RUTRACKER_PASSWORD")?.into(),
        base_url: env::var("RUTRACKER_BASEURL")?,
        login_path: env::var("RUTRACKER_LOGIN_PATH")?,
        search_path: env::var("RUTRACKER_SEARCH_PATH")?,
    };

    let mut rt = RuTrackerProvider::new(rt_config)?;

    rt.login().await?;


    if rt.is_session_active().await? {
        println!("Logged in");
    }

    rt.search("Game of thrones".into(), vec![]).await?;


    Ok(())
}
