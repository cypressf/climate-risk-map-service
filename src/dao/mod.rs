use super::model::{County, State};

mod county_dao;
pub mod database;
mod state_dao;

pub type Database<'c> = database::Database<'c>;
pub type Table<'c, T> = database::Table<'c, T>;
