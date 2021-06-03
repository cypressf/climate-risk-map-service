use super::County;
use super::Table;

impl<'c> Table<'c, County> {
    pub async fn by_id(&self, id: &str) -> Result<County, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`
            FROM `county`
            WHERE `id` = ?"#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }
}
