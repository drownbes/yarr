mod kinozaltv;
pub mod rutracker;
mod select_utils;

use std::future::Future;

struct SearchResult;
struct CategoriesResult;

trait Provider: Send + Sync + 'static {
    fn search(&self) -> impl Future<Output = anyhow::Result<SearchResult>>;
    fn categories(&self) -> impl Future<Output = anyhow::Result<CategoriesResult>>;
}

struct RuTracker;

impl Provider for RuTracker {
    async fn search(&self) -> anyhow::Result<SearchResult> {
        todo!()
    }

    async fn categories(&self) -> anyhow::Result<CategoriesResult> {
        todo!()
    }
}
