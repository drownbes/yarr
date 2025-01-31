mod kinozaltv;
pub mod rutracker;
pub mod search_results;
mod select_utils;

use std::future::Future;

use search_results::SearchResults;

pub struct CategoriesResult;

trait Provider: Send + Sync + 'static {
    fn search(&self) -> impl Future<Output = anyhow::Result<SearchResults>>;
    fn categories(&self) -> impl Future<Output = anyhow::Result<CategoriesResult>>;
}
