use regex::Regex;
use scraper::{Html, Selector};

pub struct IndexPage {
    document: Html,
    raw_html: String
}

impl IndexPage {
    pub fn new(html_str: &str) -> IndexPage {
        IndexPage {
            document: Html::parse_document(html_str),
            raw_html: html_str.to_string()
        }
    }
    pub fn is_logged(&self) -> anyhow::Result<bool> {
        let is_logged_sel = Selector::parse("#logged-in-username").expect("Invalid selector");
        Ok(self.document.select(&is_logged_sel).next().is_some())
    }

    pub fn get_form_token(&self) -> Option<String> {
        let re = Regex::new(r#"form_token:\s*'([a-f0-9]+)'"#).unwrap();
        let caps = re.captures(&self.raw_html)?;
        caps.get(1).map(|m| m.as_str().to_string())
    }
}
