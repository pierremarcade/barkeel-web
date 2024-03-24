use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2::Error;

pub struct Connector;

impl Connector {
    pub fn connect(database_url: &String) -> Result<Pool<ConnectionManager<MysqlConnection>>, Error> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager);
        pool
    }
}

pub struct Database {
    pub pool: MysqlConnection
}

impl Database {
    pub fn new(pool: MysqlConnection) -> Self {
        Database { pool }
    }
}