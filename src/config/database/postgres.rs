use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2::Error;

pub struct Connector;

impl Connector {
    pub fn connect(database_url: &String) -> Result<Pool<ConnectionManager<PgConnection>>, Error> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager);
        pool
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>
}

impl Database {
    pub fn new(pool:  Pool<ConnectionManager<PgConnection>>) -> Self {
        Database { pool }
    }
}