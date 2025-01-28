use scraper::{ElementRef, Selector};

pub trait SelectUtils<'a> {
    fn get_el(&self) -> ElementRef<'a>;

    fn get_inner_text(&self, selector: &str) -> Option<String> {
        let el = self.get_el();
        let sel = Selector::parse(selector).expect("Invalid selector");
        let texts: Vec<String> = el
            .select(&sel)
            .next()?
            .text()
            .map(|x| x.to_owned())
            .collect();
        Some(texts.join("").trim().to_string())
    }

    fn get_number(&self, selector: &str) -> Option<i64> {
        let sel = Selector::parse(selector).expect("Invalid selector");
        let text = self.get_el().select(&sel).next()?.text().next()?;
        text.parse().ok()
    }

    fn get_attr(&self, selector: &str, attr: &str) -> Option<String> {
        let sel = Selector::parse(selector).expect("Invalid selector");
        Some(
            self.get_el()
                .select(&sel)
                .next()?
                .value()
                .attr(attr)?
                .to_owned(),
        )
    }
}
