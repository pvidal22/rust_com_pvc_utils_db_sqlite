//! # Objective
//! Module for managing a the SQLite DB access
//! # History
//! version = "1.0.0" # v.20250126_01 First commit
//! version = "1.0.1" # v.20250126_02 Forced to have the DB name.
//! version = "1.0.2" # v.20250127_01 Made functions to simplify the reading of the fields.
//! version = "1.0.3" # v.20250130_01 Made functions to simplify the reading of the fields.
//! version = "1.0.4" # v.20250131_01 Use the generic types for the managing the SDBQueryReturn
//! version = "1.0.5" # v.20250131_02 The prepare connection will return as well information about the fields.
//! version = "1.0.6" # v.20250201_01 Change the location of the trait TToVectorOfString

pub mod m_db_connection;
pub mod m_db_field;
pub mod m_db_query_return;
pub mod m_db;
pub mod m_db_record_as_vector_of_strings;
pub mod m_traits;

#[derive(Clone, Debug, PartialEq)]
pub enum EDBError
{
    DBConnectionNotAvailable(String),
    DBPrepareQuery(String),
    DBExecutingQuery(String),
    DBTransactionCreation(String),
    DBTransactionCommit(String),
    DBGettingFieldType(String),
    DBRusqlitepopulated(String),
    DBQueryReturnedNoRows,
}

impl From<rusqlite::Error> for EDBError
{
    fn from(value: rusqlite::Error) -> Self 
    {
        Self::DBRusqlitepopulated(value.to_string())
    }
}
