use scraper::{selectable::Selectable, ElementRef, Html, Selector};

pub struct FilesTreePage {
    document: Html,
}

impl FilesTreePage {
    pub fn new(html_str: &str) -> FilesTreePage {
        FilesTreePage {
            document: Html::parse_document(html_str),
        }
    }

    pub fn get_files(&self) -> Option<Vec<FileOrDir>> {
        let sel = Selector::parse(".ftree").expect("Invalid selector");
        let file_tree = self.document.select(&sel).next()?;
        Some(map_tree(file_tree))
    }
}

fn map_tree(el: ElementRef) -> Vec<FileOrDir> {
    el.child_elements()
        .map(|x| {
            dbg!(x);
            if x.value()
                .has_class("dir", scraper::CaseSensitivity::CaseSensitive)
            {
                let name = x.text().next().unwrap().to_string();
                let ch_sel = Selector::parse("ul").expect("Invalid selector");
                FileOrDir::Dir(Dir {
                    name,
                    contents: map_tree(x.select(&ch_sel).next().unwrap()),
                })
            } else {
                let name = x.text().next().unwrap().to_string();
                FileOrDir::File(File {
                    name,
                    size: (99, "hehe".to_string()),
                })
            }
        })
        .collect()
}

#[derive(Debug)]
pub enum FileOrDir {
    File(File),
    Dir(Dir),
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: (i64, String),
}

#[derive(Debug)]
pub struct Dir {
    name: String,
    contents: Vec<FileOrDir>,
}

#[cfg(test)]
mod tests {
    use crate::providers::rutracker::test_helpers;

    use super::*;

    fn read_fixture() -> String {
        test_helpers::read_fixture("./fixtures/files_tree.html")
    }

    #[test]
    fn test_get_files() {
        let page = read_fixture();

        let tp = FilesTreePage::new(&page);

        let r = tp.get_files();
        dbg!(r);
    }
}
