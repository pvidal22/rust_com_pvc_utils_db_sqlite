//! Module to handle the database access.
//! # History
//! * Version 20250125_01
//!     First version
use com_pvc_utils_logs::{log_error, log_info, log_warning, m_slogs_std::*};
use com_pvc_utils_pool::m_pool::SPool;
use rusqlite::Connection;
use crate::{m_db_connection::SDBConnection, EDBError};

const DB_INITIAL_CONN: usize = 1;
const DB_MAX_CONN: usize = 20;

pub struct SDB
{
    db_pool: SPool<SDBConnection>,
    db_name: String,
}

impl SDB
{
    /// Method to initiliaze the DB pool that will be used afterwards during the rest of the execution.
    /// # Arguments
    /// * test_db_name should only be used for testing, otherwise, use None
    pub fn initialize_db_pool(db_name: &str) -> Self
    {
        log_info!("Making connection");
        log_info!(&format!("DB Name: {db_name}"));
        // let a = DB_NAME.write();
        // if let Err(e) = a
        // {
        //     let msg = format!("Problem assigning the Database name by lock: {}", e.to_string());
        //     log_error!(&msg);
        //     panic!("{}", msg);
        // }

        // let mut a = a.unwrap();
        // *a = db_name.to_string();
        // drop(a);
    
        // Creating the rusqlite connections pool
        let mut conn_pool = Vec::new();        
        for lii in 0..DB_INITIAL_CONN
        {
            conn_pool.push(SDBConnection::new(
                SDB::create_new_connection(&db_name, true).unwrap()
            ));
            log_info!(format!("Created connection: {} out of {}", lii, DB_MAX_CONN))
        }
        let pool = SPool::new(20, conn_pool, true);
        SDB
        {
            db_pool: pool,
            db_name: db_name.to_owned(),
        }
    }

    fn create_new_connection(db_name: &str, panic_when_error: bool) -> Result<Connection, EDBError>
    {
        let conn = Connection::open(db_name);
        if let Err(e) = conn
        {
            log_error!(&format!("Error connecting to DB {}", e.to_string()));
            if panic_when_error
                {panic!("{}", e);}
            return Err(EDBError::SERVERDBConnectionNotAvailable(String::from("Connection not available")));
        }
        let a = conn.unwrap();
        Ok(a)
    }

    /// Method to return the reference to the Db pool
    pub fn get_db_pool(&self) -> &SPool<SDBConnection>
    {
        &self.db_pool
    }

    /// Method to return a single connection handling the errors.
    pub fn get_db_connection(&self) -> Result<SDBConnection, EDBError>
    {
        let connection = self.db_pool.get_item();
        match connection
        {
            Some(c) => return Ok(c),
            None => 
            {                
                return Ok(SDBConnection::new(SDB::create_new_connection(&self.db_name, false)?));
            }            
        };
    }

    /// Method to retrieve a connection.
    pub fn release_connection(&self, connection: SDBConnection)
    {        
        let lb = self.db_pool.release_item(connection);
        if !lb
        {
            log_warning!("Connection was not properly released");
        }
    }
}
