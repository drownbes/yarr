use std::{io::BufReader, sync::Arc};

use anyhow::anyhow;
use reqwest_cookie_store::{CookieStoreMutex, CookieStore};
use scraper::{element_ref::Select, selectable::Selectable, ElementRef, Html, Selector};
//https://rutracker.net/forum/login.php
//
use secrecy::{ExposeSecret, SecretString};
use tokio::sync::Mutex;

use crate::repos::cookies_repo::CookiesRepo;



pub struct RuTrackerConfig {
    pub login: SecretString,
    pub password: SecretString,
    pub base_url: String,
    pub login_path: String,
    pub search_path: String,
    pub provider_id: String
}


pub struct RuTrackerProvider {
    cookie_store: Arc<CookieStoreMutex>,
    client: reqwest::Client,
    config: RuTrackerConfig,
    cookie_repo: Arc<Mutex<CookiesRepo>>,
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
    pub async fn new(config: RuTrackerConfig, cookie_repo: Arc<Mutex<CookiesRepo>>) -> anyhow::Result<RuTrackerProvider> {
        let pi = config.provider_id.clone();
        let res = cookie_repo.lock().await.get(&pi).await?;

        let cookie_store = match res {
            Some(cookie) => { 
                dbg!("Stored cookie {}", &cookie);
                reqwest_cookie_store::CookieStore::load_json(cookie.as_bytes()).unwrap()
            }
            None => reqwest_cookie_store::CookieStore::new(None)
        };

        let cookie_store = std::sync::Arc::new(CookieStoreMutex::new(cookie_store));

        let client = reqwest::ClientBuilder::new()
            .cookie_provider(cookie_store.clone())
            .build()?;
        Ok(RuTrackerProvider {
            cookie_repo,
            cookie_store,
            client,
            config
            
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

    async fn persist_cookies(&self) -> anyhow::Result<()> {
        let mut buf = vec![];
        let cookie_str = {
            let store = self.cookie_store.lock().unwrap();
            store.save_json(&mut buf).expect("Failed to save cookie");
            String::from_utf8(buf)
        }?;
        dbg!("cookie to write");
        dbg!(&cookie_str);
        self.cookie_repo.lock().await.insert(&self.config.provider_id, cookie_str).await?;
        Ok(())
    }

    pub async fn login(&mut self) -> anyhow::Result<()> {
        if let Ok(true) = self.is_session_active().await {
            dbg!("Already logged in");
            return Ok(())
        }
     
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

        self.persist_cookies().await?;

        Ok(())
    }

    pub fn prepend_base_url(&self, path: &str) -> String {
        format!("{}/forum/{}", self.config.base_url, path)
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
            .await?
            .text()
            .await?;

        let mut pages : Vec<RuTrackerSearchResultHtml> = vec![];

        let page = RuTrackerSearchResultHtml::new(&res);
        let mut next_url = page.get_next_page_url().map(|u| self.prepend_base_url(&u));
            
        dbg!(&next_url);

        pages.push(page);

        while let Some(url) = next_url {
            dbg!(&url);
            let html = self.client.get(url)
                .send()
                .await?
                .text()
                .await?;
            let p = RuTrackerSearchResultHtml::new(&html);
            next_url = p.get_next_page_url().map(|u| self.prepend_base_url(&u));
            pages.push(p);
        }

        Ok(())
    }
}


struct RuTrackerSearchResultHtml {
    document: Html
}

impl RuTrackerSearchResultHtml {
   fn new(html_str: &str) -> RuTrackerSearchResultHtml {
        RuTrackerSearchResultHtml {
            document: Html::parse_document(html_str)
        }
   }

    fn get_next_page_url(&self) -> Option<String> {
        let next_btn_sel = Selector::parse(".nav a.pg").expect("Invalid selector");
        let a_el = self.document.select(&next_btn_sel).last()?;
        if a_el.text().next()? != "След." {
            return None
        }
        let href = a_el.value().attr("href")?.to_owned();
        Some(href)
    }

    fn search_result_rows(&self) -> Vec<ResultRow> {
        let rows_sel = Selector::parse("tr.hl-tr").expect("Invalid selector");
        self.document.select(&rows_sel).map(ResultRow::new).collect()
    }
}

struct ResultRow<'a> {
    el: ElementRef<'a>
}

impl<'a> ResultRow<'a> {
    fn new(el: ElementRef<'a>) -> ResultRow<'a> {
        ResultRow {
            el
        }
    }
}


