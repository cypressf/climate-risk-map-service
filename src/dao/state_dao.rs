use super::State;
use super::Table;

impl<'c> Table<'c, State> {
    pub async fn by_id(&self, id: &str) -> Result<State, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`
            FROM `state`
            WHERE `id` = ?"#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }
}
