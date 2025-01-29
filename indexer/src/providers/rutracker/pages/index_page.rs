use scraper::{Html, Selector};

pub struct IndexPage {
    document: Html,
}

impl IndexPage {
    pub fn new(html_str: &str) -> IndexPage {
        IndexPage {
            document: Html::parse_document(html_str),
        }
    }
    pub fn is_logged(&self) -> anyhow::Result<bool> {
        let is_logged_sel = Selector::parse("#logged-in-username").expect("Invalid selector");
        Ok(self.document.select(&is_logged_sel).next().is_some())
    }
}
