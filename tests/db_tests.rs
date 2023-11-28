use std::error::Error;

use odbc_api::{Environment, ConnectionOptions};

#[test]
fn test_db_connection() -> Result<(), Box<dyn Error>> {
    let env = Environment::new()?;
    let con_str = "Driver={ODBC Driver 17 for SQL Server};Server=GHIEGIE;Database=TestDataBase;Trusted_Connection=yes;";
    let mut conn = env.connect_with_connection_string(con_str, ConnectionOptions::default())?;

    Ok(())
}

fn test_deadpool() -> Result<(), Box<dyn Error>> {


    Ok(())
}