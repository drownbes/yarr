pub mod client;
pub mod pages;
use super::{search_results::SearchResults, CategoriesResult, Provider};
#[cfg(test)]
pub mod test_helpers;

struct RuTrackerProvider {
    client: client::RuTrackerClient
}

impl Provider for RuTrackerProvider {
    async fn search(&self) -> anyhow::Result<SearchResults> {
        todo!()
    }

    async fn categories(&self) -> anyhow::Result<CategoriesResult> {
        todo!()
    }
}
