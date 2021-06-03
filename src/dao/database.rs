use super::{County, State};
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
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = MySqlPool::connect(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            state: Arc::from(Table::new(pool.clone())),
            county: Arc::from(Table::new(pool.clone())),
        }
    }
}
