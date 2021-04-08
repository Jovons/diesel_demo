use diesel::*;
use dotenv::dotenv;
use std::env;

pub fn connection_without_transaction() -> TestConnection {
    dotenv().ok();
    let connection_url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests");
    MysqlConnection::establish(&connection_url).unwrap()
}
pub type TestConnection = MysqlConnection;

pub fn connection() -> TestConnection {
    let result = connection_without_transaction();
    #[cfg(feature = "sqlite")]
    result.execute("PRAGMA foreign_keys = ON").unwrap();
    result.begin_test_transaction().unwrap();
    result
}
