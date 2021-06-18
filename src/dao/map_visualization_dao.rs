use super::MapVisualization;
use super::Table;

impl<'c> Table<'c, MapVisualization> {
    pub async fn by_dataset(&self, dataset: &str) -> Result<MapVisualization, sqlx::Error> {
        sqlx::query_as("SELECT * FROM map_visualization WHERE dataset = ?")
            .bind(dataset)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn by_id(&self, id: &str) -> Result<MapVisualization, sqlx::Error> {
        sqlx::query_as("SELECT * FROM map_visualization WHERE id = ?")
            .bind(id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn all(&self) -> Result<Vec<MapVisualization>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM map_visualization")
            .fetch_all(&*self.pool)
            .await
    }
}
