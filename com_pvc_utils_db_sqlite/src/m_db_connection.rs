use rusqlite::{Connection, Row, Statement, Transaction};

use crate::{m_db_field::{EDBFieldType, SDBField}, m_db_query_return::SDBQueryReturn, m_db_record_as_vector_of_strings::SRecordAsVectorOfStrings, EDBError};

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
    /// Method to prepare a query and obtain the stmt to execute it with parameters and also the column names,
    /// but without the field type and without records.
    pub fn prepare_stmt_for_query<'a>(&'a self, sql: &str) -> Result<(Statement<'a>, Vec<String>), EDBError>
    {
        let stmt = self.connection.prepare(sql)
            .map_err(|e| EDBError::DBPrepareQuery(e.to_string()))?;

        let columns = obtain_column_names_from_statement(&stmt);        
        Ok((stmt, columns))
    }

    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize, EDBError>
    where P: rusqlite::Params
    {
        self.connection.execute(sql, params)
            .map_err(|e| EDBError::DBExecutingQuery(e.to_string()))        
    }

    pub fn get_single_query_row_as_vector_of_strings<P>(&self, sql: &str, params: P) -> Option<SRecordAsVectorOfStrings>
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
pub fn get_row_as_vector_of_strings(row: &Row) -> Result<SRecordAsVectorOfStrings, EDBError>
{
    row_as_vector_of_strings(row)
        .map_err(|e| EDBError::DBRusqlitepopulated(e.to_string()))
}

fn row_as_vector_of_strings(row: &Row) -> Result<SRecordAsVectorOfStrings, rusqlite::Error>
{
    let mut row_strings = Vec::new();
    let mut lii = 0; // Starting from field 0
    loop
    {            
        let raw_value = row.get_ref(lii);
        if raw_value.is_err()
            {break;}
        //log_debug!(format!("Raw_value_in_single_query: {:?}", raw_value));            
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
        Ok(SRecordAsVectorOfStrings::new(row_strings))
    }        
}

// Method to return the query records with as SDBQueryReturn. 
// If no record is returned will be issued a EDBError:QueryReturnednoRows
pub fn execute_query_without_parameters_as_vector_of_strings(conn: &SDBConnection, sql: &str) -> Result<SDBQueryReturn<SRecordAsVectorOfStrings>, EDBError>
{    
    let mut query_return = SDBQueryReturn::default();
    let (mut stmt, column_names) = conn.prepare_stmt_for_query(sql)?;    
    let mut rows = stmt.query(())?;
    let mut first_time = true;    
    let mut records = Vec::new();
    while let Some(row) = rows.next()?
    {
        if first_time
        {
            let fields = get_field_types(&row, &column_names)?;            
            query_return.set_fields(fields);
            first_time = false;
        }
        let values = row_as_vector_of_strings(&row)
            .map_err(|e| EDBError::DBRusqlitepopulated(e.to_string()))?;
        records.push(values);
    }

    if first_time // Not a single record
    {
        return Err(EDBError::DBQueryReturnedNoRows);
    }

    query_return.set_records(records);
    
    Ok(query_return)
    
}

fn obtain_column_names_from_statement(stmt: &Statement) -> Vec<String>
{
    let a = stmt.column_names()
        .iter().map(|x| x.to_string())
        .collect::<Vec<_>>();
    a
}

pub fn get_field_types(row: &Row, column_names: &Vec<String>) -> Result<Vec<SDBField>, EDBError>
{
    let mut fields = Vec::new();
    for idx_column in 0..column_names.len()
    {
        let column = row.get_ref(idx_column).unwrap();
       
        let field = match column
            {
                rusqlite::types::ValueRef::Null => SDBField::new(idx_column, column_names.get(idx_column).unwrap(), EDBFieldType::Null),
                rusqlite::types::ValueRef::Integer(_) => SDBField::new(idx_column, column_names.get(idx_column).unwrap(), EDBFieldType::Integer),
                rusqlite::types::ValueRef::Real(_) => SDBField::new(idx_column, column_names.get(idx_column).unwrap(), EDBFieldType::Real),
                rusqlite::types::ValueRef::Text(_) => SDBField::new(idx_column, column_names.get(idx_column).unwrap(), EDBFieldType::Text),
                rusqlite::types::ValueRef::Blob(_) => SDBField::new(idx_column, column_names.get(idx_column).unwrap(), EDBFieldType::Blob),
            };
        fields.push(field);
    }

    Ok(fields)
}

