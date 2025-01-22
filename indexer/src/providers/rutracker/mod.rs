use std::sync::Arc;

use anyhow::anyhow;
use reqwest::cookie::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{selectable::Selectable, Html, Selector};
//https://rutracker.net/forum/login.php
//
use secrecy::{ExposeSecret, SecretString};



pub struct RuTrackerConfig {
    pub login: SecretString,
    pub password: SecretString,
    pub base_url: String,
    pub login_path: String,
    pub search_path: String,
}


pub struct RuTrackerProvider {
    cookie_store: Arc<CookieStoreMutex>,
    client: reqwest::Client,
    config: RuTrackerConfig,
    is_logged: bool
}

#[derive(serde::Serialize, Debug)]
struct LoginReq {
    login_username: String,
    login_password: String,
    login: String
}

#[derive(serde::Serialize, Debug)]
struct SearchReq {
    nm: String,
    f: Vec<u64>
}


impl RuTrackerProvider {
    pub fn new(config: RuTrackerConfig) -> anyhow::Result<RuTrackerProvider> {
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(
            reqwest_cookie_store::CookieStore::new(None)
        );
        let cookie_store = std::sync::Arc::new(cookie_store);
        let client = reqwest::ClientBuilder::new()
            .cookie_provider(cookie_store.clone())
            .build()?;
        Ok(RuTrackerProvider {
            cookie_store,
            client,
            config,
            is_logged: false
            
        })
    }

    pub async fn is_session_active(&self) -> anyhow::Result<bool> {
        let store = self.cookie_store.lock().unwrap();
        for c in store.iter_any() {
            println!("{:?}", c);
        }

        let session_cookie = store.get("rutracker.net", "/forum/", "bb_session");
        Ok(session_cookie.is_some())
    }

    pub async fn is_logged(html: String) -> anyhow::Result<bool> {
        let document = Html::parse_document(&html);
        let is_logged_sel = Selector::parse("#logged-in-username").expect("Invalid selector");
        Ok(document.select(&is_logged_sel).next().is_some())
    }

    fn login_path(&self) -> String {
        format!("{}{}", self.config.base_url, self.config.login_path)
    }

    fn search_path(&self) -> String {
        format!("{}{}", self.config.base_url, self.config.search_path)
    }


    pub async fn login(&mut self) -> anyhow::Result<()> {
        let req = LoginReq {
            login_username: self.config.login.expose_secret().to_string(),
            login_password: self.config.password.expose_secret().to_string(),
            login: "вход".to_string()
        };
        let _res = self.client
            .post(self.login_path())
            .form(&req)
            .send()
            .await?;

        Ok(())
    }

    pub async fn search(&self, query: String, categories: Vec<u64>) -> anyhow::Result<()> {
        let cs : String = categories.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        let mut req = vec![
            ("nm", query)
        ];
        if !cs.is_empty() {
            req.push(("f", cs));
        }

        let res = self.client.get(self.search_path())
            .query(&req)
            .send()
            .await?;

        dbg!(&res);

        Ok(())
    }
}
