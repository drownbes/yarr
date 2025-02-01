use crate::providers::select_utils::SelectUtils;
use scraper::{ElementRef, Html, Selector};

pub struct SearchResultPage {
    document: Html,
}

impl SearchResultPage {
    pub fn new(html_str: &str) -> SearchResultPage {
        SearchResultPage {
            document: Html::parse_document(html_str),
        }
    }

    pub fn get_next_page_href(&self) -> Option<String> {
        let next_btn_sel = Selector::parse(".nav a.pg").expect("Invalid selector");
        let a_el = self.document.select(&next_btn_sel).last()?;
        if a_el.text().next()? != "След." {
            return None;
        }
        let href = a_el.value().attr("href")?.to_owned();
        Some(href)
    }

    pub fn get_search_result_rows(&self) -> Vec<ResultRow> {
        let rows_sel = Selector::parse("tr.hl-tr").expect("Invalid selector");
        self.document
            .select(&rows_sel)
            .map(ResultRow::new)
            .collect()
    }

    pub fn get_categories(&self) -> Vec<CategoryGroup> {
        let sel_optgroups = Selector::parse("#fs-main optgroup").expect("Invalid selector");
        self.document
            .select(&sel_optgroups)
            .flat_map(|grp| {
                let label = grp.attr("label")?.replace("\u{a0}", "").trim().to_owned();
                let sel = Selector::parse("option").expect("Invalid selector");
                let cts = grp
                    .select(&sel)
                    .flat_map(|opt| {
                        Some(Category {
                            id: opt.value().attr("value")?.parse().ok()?,
                            label: opt
                                .text()
                                .next()?
                                .replace("\u{a0}", "")
                                .replace("|-", "")
                                .trim()
                                .to_owned(),
                        })
                    })
                    .collect();
                Some(CategoryGroup {
                    label,
                    categories: cts,
                })
            })
            .collect()
    }

    pub fn get_topic_files(&self, id: i64) -> Option<()> {
        Some(())
    }
}

#[derive(Debug)]
pub struct CategoryGroup {
    label: String,
    categories: Vec<Category>,
}

#[derive(Debug)]
pub struct Category {
    id: i64,
    label: String,
}

pub struct ResultRow<'a> {
    el: ElementRef<'a>,
}

impl<'a> SelectUtils<'a> for ResultRow<'a> {
    fn get_el(&self) -> ElementRef<'a> {
        self.el
    }
}

impl<'a> ResultRow<'a> {
    fn new(el: ElementRef<'a>) -> ResultRow<'a> {
        ResultRow { el }
    }

    pub fn get_id(&self) -> Option<i64> {
        self.el.value().attr("data-topic_id")?.parse().ok()
    }

    pub fn get_topic(&self) -> Option<String> {
        self.get_inner_text("td .t-title a")
    }

    pub fn get_link(&self) -> Option<String> {
        self.get_attr("td .t-title a", "href")
    }

    pub fn get_size(&self) -> Option<(String, String)> {
        let raw_size = self
            .get_inner_text("td .tor-size a")?
            .replace("↓", "")
            .trim()
            .to_string();
        let mut i = raw_size.split("\u{a0}");
        Some((i.next()?.to_owned(), i.next()?.to_owned()))
    }

    pub fn get_seeders(&self) -> Option<i64> {
        self.get_number("td .seedmed")
    }

    pub fn get_leechers(&self) -> Option<i64> {
        self.get_number("td .leechmed")
    }

    pub fn get_category(&self) -> Option<(i64, String)> {
        let ct = self.el.select(&Selector::parse("td.f-name-col a").unwrap()).next()?;
        let text = ct.text().next()?.to_owned();
        let href = ct.value().attr("href")?.to_owned();
        let re = regex::Regex::new(r#"f=(.+)$"#).unwrap();
        let id : i64 = re.captures(&href)?.get(1)?.as_str().parse().ok()?;
        Some((id, text))
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::rutracker::test_helpers;

    use super::*;
    
    #[test]
    fn test_category_regex() {
        let href = "tracker.php?f=1449";
        let re = regex::Regex::new(r#"f=(.+)$"#).unwrap();
        let id  : i64= re.captures(href).unwrap().get(1).unwrap().as_str().parse().unwrap();
        dbg!(id);
    }

    fn read_fixture() -> String {
        test_helpers::read_fixture("./fixtures/search_page.html")
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
    fn test_get_topic() {
        let html_str = read_fixture();
        let nd = SearchResultPage::new(&html_str);
        let rows = nd.get_search_result_rows();
        let fr = rows.first().unwrap();
        dbg!(fr.get_id().unwrap());
        dbg!(fr.get_topic().unwrap());
        dbg!(fr.get_link());
        dbg!(fr.get_size());
        dbg!(fr.get_seeders());
        dbg!(fr.get_leechers());
    }

    #[test]
    fn test_get_categories() {
        let html_str = read_fixture();
        let nd = SearchResultPage::new(&html_str);
        //dbg!(nd.get_categories());
    }
}
