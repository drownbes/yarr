use scraper::{selectable::Selectable, ElementRef, Html, Selector};

pub struct SearchResultPage {
    document: Html
}

impl SearchResultPage {
   pub fn new(html_str: &str) -> SearchResultPage {
        SearchResultPage {
            document: Html::parse_document(html_str)
        }
   }

    pub fn get_next_page_href(&self) -> Option<String> {
        let next_btn_sel = Selector::parse(".nav a.pg").expect("Invalid selector");
        let a_el = self.document.select(&next_btn_sel).last()?;
        if a_el.text().next()? != "След." {
            return None
        }
        let href = a_el.value().attr("href")?.to_owned();
        Some(href)
    }

    pub fn get_search_result_rows(&self) -> Vec<ResultRow> {
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

    fn get_topic(&self) -> Option<String> {
        let topic_sel = Selector::parse("td.t-title a").expect("Invalid selector");
        let topic : Vec<String> = self.el.select(&topic_sel).next()?.text().map(|x| x.to_owned()).collect();
        Some(topic.join("").trim().to_string())
    }

}


#[cfg(test)]
mod tests {
    use encoding_rs::Encoding;

    use super::*;

    fn read_fixture() -> String {
        let html_str = include_bytes!("fixtures/search_page.html");
        let enc = Encoding::for_label(b"Windows-1251").unwrap();
        let (html_str,_,_) = enc.decode(html_str);
        html_str.to_string()
    }

    #[test]
    fn test_get_next_page_href() {
        let html_str = read_fixture();
        let nd = SearchResultPage::new(&html_str);
        let href = nd.get_next_page_href().unwrap();
        assert_eq!("tracker.php?search_id=0MFnL4CXXb1l&start=50", href);
    }

    #[test]
    fn test_get_search_result_rows() {
        let html_str = read_fixture();
        let nd = SearchResultPage::new(&html_str);

        let rows = nd.get_search_result_rows();
        assert_eq!(rows.len(), 50);
    }

    #[test]
    fn test_get_row
}
