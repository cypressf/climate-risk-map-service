use super::model::{County, Data, State};

mod county_dao;
mod data_dao;
pub mod database;
mod state_dao;

pub type Database<'c> = database::Database<'c>;
pub type Table<'c, T> = database::Table<'c, T>;
