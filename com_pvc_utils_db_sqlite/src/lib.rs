//! # Objective
//! Module for managing a the SQLite DB access
//! # History
//! version = "1.0.0" # v.20250126_01 First commit

pub mod m_db_connection;
pub mod m_db;

pub type TypeDBRowOfStrings = Vec<String>;

#[derive(Clone, Debug)]
pub enum EDBError
{
    SERVERDBConnectionNotAvailable(String),
    SERVERDBPrepareQuery(String),
    SERVERDBExecutingQuery(String),
    SERVERDBTransactionCreation(String),
    SERVERDBTransactionCommit(String),
}