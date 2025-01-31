use super::{CategoriesResult, Provider};

pub mod client;
pub mod pages;
use std::future::Future;
use super::search_results::SearchResults;
#[cfg(test)]
pub mod test_helpers;

struct RuTrackerProvider;

impl Provider for RuTrackerProvider {
    async fn search(&self) -> anyhow::Result<SearchResults> {
        todo!()
    }

    async fn categories(&self) -> anyhow::Result<CategoriesResult> {
        todo!()
    }
}
