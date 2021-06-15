use super::Data;
use super::Table;

impl<'c> Table<'c, Data> {
    pub async fn by_id(&self, id: &str) -> Result<Vec<Data>, sqlx::Error> {
        sqlx::query_as("SELECT state_id, county_id, value FROM county_data WHERE dataset = ?")
            .bind(id)
            .fetch_all(&*self.pool)
            .await
    }
}
