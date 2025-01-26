use com_pvc_utils_logs::{log_info, log_warning, m_slogs_std::*, m_tlogs::ELogLevel};

use crate::{m_db::SDB, m_db_connection};

#[test]
fn db_creation_01()
{
    let test_name = String::from("db_creation_01");
    let logfile = format!("{test_name}.log");
    let logfolder = format!("./logs-{test_name}/");
    let db_name = format!("{test_name}.db");
    let _ = log_init("records_creation_01", &logfile, &logfolder, ELogLevel::Debug, true);
    log_info!(&format!("Performing test: {}", test_name));

    let db_path = std::path::Path::new(&db_name);
    let log_path = std::path::Path::new(&logfolder);

    if db_path.exists()
    {
        log_warning!(&format!("Removing file {}", db_name));
        std::fs::remove_file(db_path).unwrap(); // TO make sure it panics with a "clear" error.
    }

    let _ = SDB::initialize_db_pool(Some(&db_name));

    assert_eq!(true, db_path.exists());
    
    let _ = std::fs::remove_file(db_path).unwrap();
    _ = std::fs::remove_dir_all(log_path);

}

#[test]
fn db_table_creation_01()
{
    let test_name = String::from("db_table_creation_01");
    let logfile = format!("{test_name}.log");
    let logfolder = format!("./logs-{test_name}/");
    let db_name = format!("{test_name}.db");
    let _ = log_init("table_creation_01", &logfile, &logfolder, ELogLevel::Debug, true);

    log_info!(&format!("Performing test: {}", test_name));

    let db_path = std::path::Path::new(&db_name);
    let log_path = std::path::Path::new(&logfolder);

    if db_path.exists()
    {
        log_warning!(&format!("Removing file {}", db_name));
        std::fs::remove_file(db_path).unwrap(); // TO make sure it panics with a "clear" error.
    }
    
    {
        let sql = "CREATE TABLE 'tsettings' (
            'id'	TEXT NOT NULL UNIQUE,
            'value'	TEXT,
            PRIMARY KEY('id')
        );";

        let db = SDB::initialize_db_pool(Some(&db_name));
        assert_eq!(true, db_path.exists());
        let mut conn = db.get_db_connection().unwrap();
        let tr = conn.start_transaction().unwrap();
        let _ = tr.execute(sql, ()).unwrap();
        m_db_connection::commit_transaction(tr).unwrap();
        db.release_connection(conn);
    }

    _ = std::fs::remove_file(db_path);
    _ = std::fs::remove_dir_all(log_path);
}

#[test]
fn db_record_creation_01()
{
    let test_name = String::from("db_record_creation_01");
    let logfile = format!("{test_name}.log");
    let logfolder = format!("./logs-{test_name}/");
    let db_name = format!("{test_name}.db");
    let _ = log_init("record_creation_01", &logfile, &logfolder, ELogLevel::Debug, true);

    log_info!(&format!("Performing test: {}", test_name));

    let db_path = std::path::Path::new(&db_name);
    let log_path = std::path::Path::new(&logfolder);

    if db_path.exists()
    {
        log_warning!(&format!("Removing file {}", db_name));
        std::fs::remove_file(db_path).unwrap(); // TO make sure it panics with a "clear" error.
    }

    {
        let db = SDB::initialize_db_pool(Some(&db_name));
        let mut conn_table = db.get_db_connection().unwrap();
        let mut conn_record = db.get_db_connection().unwrap();
        let conn_read = db.get_db_connection().unwrap();
        {
            let sql = "CREATE TABLE 'tsettings' (
                'id'	TEXT NOT NULL UNIQUE,
                'value'	TEXT,
                PRIMARY KEY('id')
            );";
            
            assert_eq!(true, db_path.exists());            
            let tr_table = conn_table.start_transaction().unwrap();
            tr_table.execute(sql, ()).unwrap();
            m_db_connection::commit_transaction(tr_table).unwrap();

            let sql = "insert into tsettings (id, value) values ('test1', 'test2')";                
            let tr_record = conn_record.start_transaction().unwrap();
            tr_record.execute(sql, ()).unwrap();
            m_db_connection::commit_transaction(tr_record).unwrap();
                        
            let sql = "select * from tsettings where id = 'test3'";
            let rst = conn_read.get_single_query_row_as_vector_of_strings(sql, ());
            assert_eq!(rst, None);

            let sql = "select id, value from tsettings where id = 'test1'";
            let value = conn_read.get_single_query_row_as_vector_of_strings(sql, ()).unwrap();
            let value = value.get(1).unwrap();
            assert_eq!(value, "test2");
            db.release_connection(conn_table);
            db.release_connection(conn_record);
            db.release_connection(conn_read);
        }
    }
    
    let _ = std::fs::remove_file(db_path).unwrap();
    let _ = std::fs::remove_dir_all(log_path).unwrap();
}