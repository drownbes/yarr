#[cfg(test)]
mod tests {
    use super::*;
    use magnet_url::Magnet;

    const MURL: &str = "magnet:?xt=urn:btih:d140c9ab95568b06bae1997ab1fe44d935e4ecf5&tr=http%3A%2F%2Frutracker.ru%2Fbt%2Fannounce.php%3Fuk%3D56WtM1NE1U";



    #[test]
    fn test_name() {
        let mu = Magnet::new(MURL).unwrap();
        dbg!(mu);
    }
}

