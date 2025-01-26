use com_pvc_utils_logs::{log_debug, m_slogs_std::*};
use rusqlite::{Connection, Statement, Transaction};

use crate::{EDBError, TypeDBRowOfStrings};

#[cfg(test)]
mod tests;

pub struct SDBConnection
{
    connection: Connection,
}

impl SDBConnection
{
    pub fn new(connection: Connection) -> Self
    {
        SDBConnection{ connection }
    }

    pub fn get_raw_connection(&self) -> &Connection
    {
        &self.connection
    }

    #[allow(dead_code)]
    pub fn prepare_stmt_for_query<'a>(&'a self, sql: &str) -> Result<Statement<'a>, EDBError>
    {
        let stmt = self.connection.prepare(sql)
            .map_err(|e| EDBError::SERVERDBPrepareQuery(e.to_string()))?;

        Ok(stmt)
    }

    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize, EDBError>
    where P: rusqlite::Params
    {
        self.connection.execute(sql, params)
            .map_err(|e| EDBError::SERVERDBExecutingQuery(e.to_string()))        
    }

    pub fn get_single_query_row_as_vector_of_strings<P>(&self, sql: &str, params: P) -> Option<TypeDBRowOfStrings>
    where P: rusqlite::Params
    {
        let a = self.connection.query_row(sql, params
            , |row| 
        {            
            let mut row_strings: TypeDBRowOfStrings = Vec::new();
            let mut lii = 0; // Starting from field 1
            loop
            {
                let raw_value = row.get_ref(lii);
                log_debug!(format!("Raw_value_in_single_query: {:?}", raw_value));
                //println!("Raw_value: {:?}", raw_value);
                if raw_value.is_err() 
                    {break;}
                let raw_value = raw_value.unwrap();
                let value_final = match raw_value
                    {
                        rusqlite::types::ValueRef::Null => "null".to_string(),
                        rusqlite::types::ValueRef::Integer(value) => value.to_string(),
                        rusqlite::types::ValueRef::Real(value) => value.to_string(),
                        rusqlite::types::ValueRef::Text(_) => 
                            {
                                let value: String = row.get(lii).unwrap();
                                value
                            },
                        rusqlite::types::ValueRef::Blob(_) => "Blob".to_string(),
                    };
                row_strings.push(value_final);
                lii += 1;
            }
            if row_strings.is_empty()
            {
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
            else
            {
                Ok(row_strings)            
            }
        }).ok();
        a
    }

    /// Method to start a transaction to be used with a possible rollback.
    /// # Arguments
    pub fn start_transaction(&mut self) -> Result<Transaction, EDBError>
    {
        let tr = self.connection.transaction()
            .map_err(|e| EDBError::SERVERDBTransactionCreation(e.to_string()))?;

        Ok(tr)
    }
}

/// Method to commit a created transaction.
/// # Arguments
/// * tr: Transaction
pub fn commit_transaction(transaction: Transaction) -> Result<(), EDBError>
{
    transaction.commit()
        .map_err(|e| EDBError::SERVERDBTransactionCommit(e.to_string()))?;
    Ok(())
}
