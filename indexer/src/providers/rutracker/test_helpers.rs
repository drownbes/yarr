use std::{
    fs::{self},
    path::Path,
};

use encoding_rs::Encoding;

pub fn read_fixture(path: &str) -> String {
    let cu = std::env::current_dir().unwrap();
    let p = Path::new(file!());
    let d = p.ancestors().nth(1).unwrap();
    let p = d.join(path);
    let p = cu.ancestors().nth(1).unwrap().join(p);
    let html_str = fs::read(p).unwrap();
    let enc = Encoding::for_label(b"Windows-1251").unwrap();
    let (html_str, _, _) = enc.decode(&html_str);
    html_str.to_string()
}
