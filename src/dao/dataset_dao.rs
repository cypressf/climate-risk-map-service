use super::Dataset;
use super::Table;

impl<'c> Table<'c, Dataset> {
    pub async fn by_id(&self, id: &str) -> Result<Dataset, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `short_name`, `name`, `description`, `units`
            FROM `dataset`
            WHERE `id` = ?"#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn all(&self) -> Result<Vec<Dataset>, sqlx::Error> {
        sqlx::query_as("SELECT id, short_name, name, description, units FROM dataset")
            .fetch_all(&*self.pool)
            .await
    }
}
