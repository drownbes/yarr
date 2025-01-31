pub struct TorrentShare {
    provider_id: String,
    id: i64,
    label: String,
    link: String,
    size: (String, String),
    seeders: i64,
    leechers: i64
}

pub struct SearchResults {
    results: Vec<TorrentShare>,
}

impl SearchResults {
}
