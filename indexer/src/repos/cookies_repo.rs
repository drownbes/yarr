pub struct CookiesRepo {
    pool: sqlx::SqlitePool,
}

impl CookiesRepo {
    pub fn new(pool: sqlx::SqlitePool) -> CookiesRepo {
        CookiesRepo { pool }
    }

    pub async fn insert(&self, provider_id: &str, cookie: String) -> anyhow::Result<()> {
        sqlx::query(
            r#"
                update providers set cookie = ? where id = ?
            "#,
        )
        .bind(cookie)
        .bind(provider_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get(&self, provider_id: &str) -> anyhow::Result<Option<String>> {
        let cookie = sqlx::query_scalar(
            r#"
                select cookie from providers where id = ?
            "#,
        )
        .bind(provider_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(cookie)
    }
}
