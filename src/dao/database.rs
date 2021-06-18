use super::{County, Data, DataCategory, Dataset, MapVisualization, State};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::sync::Arc;

pub struct Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'c MySqlRow) -> Result<T, sqlx::Error>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}

pub struct Database<'c> {
    pub state: Arc<Table<'c, State>>,
    pub county: Arc<Table<'c, County>>,
    pub data: Arc<Table<'c, Data>>,
    pub dataset: Arc<Table<'c, Dataset>>,
    pub map_visualization: Arc<Table<'c, MapVisualization>>,
    pub data_category: Arc<Table<'c, DataCategory>>,
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = MySqlPool::connect(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            state: Arc::from(Table::new(pool.clone())),
            county: Arc::from(Table::new(pool.clone())),
            data: Arc::from(Table::new(pool.clone())),
            dataset: Arc::from(Table::new(pool.clone())),
            map_visualization: Arc::from(Table::new(pool.clone())),
            data_category: Arc::from(Table::new(pool.clone())),
        }
    }
}
