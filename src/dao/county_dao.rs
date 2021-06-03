use super::County;
use super::Table;

impl<'c> Table<'c, County> {
    pub async fn by_id(&self, id: &str) -> Result<Vec<County>, sqlx::Error> {
        sqlx::query_as("SELECT id, name, state FROM county WHERE id = ?")
            .bind(id)
            .fetch_all(&*self.pool)
            .await
    }
}
