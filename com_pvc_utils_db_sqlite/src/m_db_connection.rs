use com_pvc_utils_logs::{log_debug, m_slogs_std::*};
use rusqlite::{Connection, Row, Rows, Statement, Transaction};

use crate::{m_db_field::{EDBFieldType, SDBField}, m_db_query_return::TypeDBRowOfStrings, EDBError};

pub const DB_NONE_VALUE: &str = "None";
pub const DB_BLOB_VALUE: &str = "Blob";

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
            .map_err(|e| EDBError::DBPrepareQuery(e.to_string()))?;

        Ok(stmt)
    }

    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize, EDBError>
    where P: rusqlite::Params
    {
        self.connection.execute(sql, params)
            .map_err(|e| EDBError::DBExecutingQuery(e.to_string()))        
    }

    pub fn get_single_query_row_as_vector_of_strings<P>(&self, sql: &str, params: P) -> Option<TypeDBRowOfStrings>
    where P: rusqlite::Params
    {
        let a = self.connection.query_row(sql, params
            , |row| row_as_vector_of_strings(row)).ok();
        a
    }


    /// Method to start a transaction to be used with a possible rollback.
    /// # Arguments
    pub fn start_transaction(&mut self) -> Result<Transaction, EDBError>
    {
        let tr = self.connection.transaction()
            .map_err(|e| EDBError::DBTransactionCreation(e.to_string()))?;

        Ok(tr)
    }
}

/// Method to commit a created transaction.
/// # Arguments
/// * tr: Transaction
pub fn commit_transaction(transaction: Transaction) -> Result<(), EDBError>
{
    transaction.commit()
        .map_err(|e| EDBError::DBTransactionCommit(e.to_string()))?;
    Ok(())
}
pub fn get_row_as_vector_of_strings(row: &Row) -> Result<TypeDBRowOfStrings, EDBError>
{
    row_as_vector_of_strings(row)
        .map_err(|e| EDBError::DBRusqlitepopulated(e.to_string()))
}

fn row_as_vector_of_strings(row: &Row) -> Result<TypeDBRowOfStrings, rusqlite::Error>
{
    let mut row_strings: TypeDBRowOfStrings = Vec::new();
    let mut lii = 0; // Starting from field 0
    loop
    {            
        let raw_value = row.get_ref(lii);
        if raw_value.is_err()
            {break;}
        log_debug!(format!("Raw_value_in_single_query: {:?}", raw_value));            
        let value_final = match raw_value.unwrap()
            {
                rusqlite::types::ValueRef::Null => DB_NONE_VALUE.to_owned(),
                rusqlite::types::ValueRef::Integer(value) => value.to_string(),
                rusqlite::types::ValueRef::Real(value) => value.to_string(),
                rusqlite::types::ValueRef::Text(_) => row.get(lii).unwrap(), // will be string
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
}

// Method to return the query records with as SDBQueryReturn. 
// If no record is returned will be issued a EDBError:QueryReturnednoRows
pub fn execute_query_without_parameters(conn: &SDBConnection, sql: &str) -> Result<TypeDBRowOfStrings, EDBError>
{
    println!("DEBUG");
    let mut stmt = conn.prepare_stmt_for_query(sql)?;
    let number_columns = stmt.column_count();
    let column_names = stmt.column_names()
        .iter().map(|x| x.to_string())
        .collect::<Vec<_>>();
    let mut rows = stmt.query(())?;
    let mut first_time = true;    
    let mut records = 0;
    while let Some(row) = rows.next()?
    {
        records += 1;        
        if first_time
        {
            let fields = get_field_types(row, &column_names);
            first_time = false;
        }
    }
    // row_as_vector_of_strings(row)
    //     .map_err(|e| EDBError::DBRusqlitepopulated(e.to_string()))
    if records == 0
    {
        return Err(EDBError::DBQueryReturnedNoRows);
    }

    Err(EDBError::DBConnectionNotAvailable("hola".to_string()))
    
}

fn get_field_types(row: &Row, column_names: &Vec<String>) -> Result<Vec<SDBField>, EDBError>
{
    let mut fields = Vec::new();
    println!("DEBUG row: {:?}", row);
    for idx_column in 0..column_names.len()
    {
        let column = row.get_ref(idx_column).unwrap();
        
        println!("DEBUG REF_Value: {:?}", column);
        let field = match column
            {
                rusqlite::types::ValueRef::Null => SDBField::new(column_names.get(idx_column).unwrap(), EDBFieldType::Null),
                rusqlite::types::ValueRef::Integer(_) => SDBField::new(column_names.get(idx_column).unwrap(), EDBFieldType::Integer),
                rusqlite::types::ValueRef::Real(_) => SDBField::new(column_names.get(idx_column).unwrap(), EDBFieldType::Real),
                rusqlite::types::ValueRef::Text(_) => SDBField::new(column_names.get(idx_column).unwrap(), EDBFieldType::Text),
                rusqlite::types::ValueRef::Blob(_) => SDBField::new(column_names.get(idx_column).unwrap(), EDBFieldType::Blob),
            };
        fields.push(field);
    }

    Ok(fields)
}

