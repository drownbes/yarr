use anyhow::anyhow;
use reqwest_cookie_store::CookieStoreMutex;
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    providers::rutracker::pages::search_results_page::SearchResultPage,
    repos::cookies_repo::CookiesRepo,
};

use super::pages::{
    files_tree_page::{FileOrDir, FilesTreePage},
    index_page::IndexPage,
};

pub struct RuTrackerConfig {
    pub login: SecretString,
    pub password: SecretString,
    pub base_url: String,
    pub login_path: String,
    pub search_path: String,
    pub index_path: String,
    pub provider_id: String,
    pub topic_path: String,
}

pub struct RuTrackerClient {
    cookie_store: Arc<CookieStoreMutex>,
    client: reqwest::Client,
    config: RuTrackerConfig,
    cookie_repo: Arc<Mutex<CookiesRepo>>,
    form_token: Option<String>,
}

#[derive(serde::Serialize, Debug)]
struct LoginReq {
    login_username: String,
    login_password: String,
    login: String,
}

impl RuTrackerClient {
    pub async fn new(
        config: RuTrackerConfig,
        cookie_repo: Arc<Mutex<CookiesRepo>>,
    ) -> anyhow::Result<RuTrackerClient> {
        let pi = config.provider_id.clone();
        let res = cookie_repo.lock().await.get(&pi).await?;

        let cookie_store = match res {
            Some(cookie) => {
                info!("Have session cookie in db");
                reqwest_cookie_store::CookieStore::load_json(cookie.as_bytes()).unwrap()
            }
            None => {
                info!("No session cookie in db");
                reqwest_cookie_store::CookieStore::new(None)
            }
        };

        let cookie_store = std::sync::Arc::new(CookieStoreMutex::new(cookie_store));

        let client = reqwest::ClientBuilder::new()
            .cookie_provider(cookie_store.clone())
            .build()?;
        Ok(RuTrackerClient {
            cookie_repo,
            cookie_store,
            client,
            config,
            form_token: None,
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

    pub async fn is_logged(&self) -> anyhow::Result<bool> {
        let res = self
            .client
            .get(self.index_path())
            .send()
            .await?
            .text()
            .await?;
        IndexPage::new(&res).is_logged()
    }

    fn login_path(&self) -> String {
        format!("{}{}", self.config.base_url, self.config.login_path)
    }

    fn search_path(&self) -> String {
        format!("{}{}", self.config.base_url, self.config.search_path)
    }
    fn index_path(&self) -> String {
        format!("{}{}", self.config.base_url, self.config.index_path)
    }

    fn topic_path(&self) -> String {
        format!("{}{}", self.config.base_url, self.config.topic_path)
    }

    fn download_path(&self) -> String {
        format!("{}{}", self.config.base_url, "/forum/dl.php")
    }

    async fn persist_cookies(&self) -> anyhow::Result<()> {
        let mut buf = vec![];
        let cookie_str = {
            let store = self.cookie_store.lock().unwrap();
            store.save_json(&mut buf).expect("Failed to save cookie");
            String::from_utf8(buf)
        }?;
        info!("Writing cookie to db");
        self.cookie_repo
            .lock()
            .await
            .insert(&self.config.provider_id, cookie_str)
            .await?;
        Ok(())
    }

    pub async fn login(&mut self) -> anyhow::Result<()> {
        if let Ok(true) = self.is_session_active().await {
            info!("Login is noop. Already logged in");
            return Ok(());
        }

        let req = LoginReq {
            login_username: self.config.login.expose_secret().to_string(),
            login_password: self.config.password.expose_secret().to_string(),
            login: "вход".to_string(),
        };
        let _res = self
            .client
            .post(self.login_path())
            .form(&req)
            .send()
            .await?;

        self.persist_cookies().await?;

        Ok(())
    }

    pub fn prepend_base_url(&self, path: &str) -> String {
        format!("{}/forum/{}", self.config.base_url, path)
    }

    pub async fn search(&self, query: String, categories: Vec<u64>) -> anyhow::Result<()> {
        let cs: String = categories
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let mut req = vec![("nm", query)];
        if !cs.is_empty() {
            req.push(("f", cs));
        }

        let res = self
            .client
            .get(self.search_path())
            .query(&req)
            .send()
            .await?
            .text()
            .await?;

        info!("Got 1 page of search results");
        let mut pages: Vec<SearchResultPage> = vec![];
        let page = SearchResultPage::new(&res);
        let mut next_url = page.get_next_page_href().map(|u| self.prepend_base_url(&u));
        pages.push(page);

        while let Some(url) = next_url {
            let html = self.client.get(url).send().await?.text().await?;
            let p = SearchResultPage::new(&html);
            next_url = p.get_next_page_href().map(|u| self.prepend_base_url(&u));
            pages.push(p);

            info!("Got {} page of search results", pages.len());
        }

        Ok(())
    }

    pub async fn topic_files(&self, id: i64) -> anyhow::Result<Option<Vec<FileOrDir>>> {
        let req = [("t", format!("{}", id))];
        let _res = self
            .client
            .post(self.topic_path())
            .form(&req)
            .send()
            .await?
            .text()
            .await?;

        let p = FilesTreePage::new(&_res);
        Ok(p.get_files())
    }

    pub async fn torrent_file(&self, id: i64) -> anyhow::Result<String> {
        let form_token = self.form_token.clone().ok_or(anyhow!("No form_token"))?;
        let res = self
            .client
            .post(self.download_path())
            .query(&[("t", id)])
            .form(&["form_token", &form_token])
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }
}
